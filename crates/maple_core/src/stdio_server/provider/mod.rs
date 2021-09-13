
mod blines;
mod dumb_jump;
mod filer;
mod files;
mod generic_provider;
mod grep;
// mod interactive_grep;
mod recent_files;

pub use self::filer::read_dir_entries;
use crate::paths::AbsPathBuf;
use crate::searcher::blines::BlinesItem;
use crate::searcher::SearchContext;
use crate::stdio_server::handler::{
    initialize_provider, CachedPreviewImpl, Preview, PreviewTarget,
};
use crate::stdio_server::input::{InputRecorder, KeyEvent};
use crate::stdio_server::rpc::Params;
use crate::stdio_server::vim::Vim;
use anyhow::{anyhow, Result};
use filter::Query;
use icon::{Icon, IconKind};
use matcher::{Bonus, MatchScope, Matcher, MatcherBuilder};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use types::{ClapItem, MatchedItem};

pub async fn create_provider(provider_id: &str, ctx: &Context) -> Result<Box<dyn ClapProvider>> {
    let provider: Box<dyn ClapProvider> = match provider_id {
        "blines" => Box::new(blines::BlinesProvider::new()),
        "dumb_jump" => Box::new(dumb_jump::DumbJumpProvider::new()),
        "filer" => Box::new(filer::FilerProvider::new(ctx.cwd.to_path_buf())),
        "files" => Box::new(files::FilesProvider::new(ctx).await?),
        "grep" => Box::new(grep::GrepProvider::new()),
        // "interactive_grep" => Box::new(interactive_grep::InteractiveGrepProvider::new(
        // ctx.cwd.to_path_buf(),
        // )),
        "recent_files" => Box::new(recent_files::RecentFilesProvider::new()),
        _ => Box::new(generic_provider::GenericProvider::new()),
    };
    Ok(provider)
}

#[derive(Debug)]
struct SearcherControl {
    stop_signal: Arc<AtomicBool>,
    join_handle: tokio::task::JoinHandle<()>,
}

impl SearcherControl {
    fn kill(self) {
        self.stop_signal.store(true, Ordering::SeqCst);
        self.join_handle.abort();
    }
}

/// bufnr and winid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufnrWinid {
    pub bufnr: usize,
    pub winid: usize,
}

/// Provider environment initialized at invoking the provider.
///
/// Immutable once initialized.
#[derive(Debug, Clone)]
pub struct ProviderEnvironment {
    pub is_nvim: bool,
    pub provider_id: ProviderId,
    pub start: BufnrWinid,
    pub input: BufnrWinid,
    pub display: BufnrWinid,
    pub icon: Icon,
    pub matcher_builder: MatcherBuilder,
    pub debounce: bool,
    pub no_cache: bool,
    pub preview_enabled: bool,
    pub display_winwidth: usize,
    pub display_winheight: usize,
    pub start_buffer_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct ScrollFile {
    line_start: usize,
    total_lines: usize,
}

impl ScrollFile {
    fn new(line_start: usize, path: &Path) -> Result<Self> {
        Ok(Self {
            line_start,
            total_lines: utils::count_lines(std::fs::File::open(path)?)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PreviewManager {
    scroll_file: Option<ScrollFile>,
    scroll_offset: i32,
    current_preview_target: Option<PreviewTarget>,
    preview_cache: Arc<RwLock<HashMap<PreviewTarget, Preview>>>,
}

impl PreviewManager {
    const SCROLL_SIZE: i32 = 10;

    pub fn new() -> Self {
        Self {
            scroll_file: None,
            scroll_offset: 0,
            current_preview_target: None,
            preview_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn cached_preview(&self, preview_target: &PreviewTarget) -> Option<Preview> {
        let preview_cache = self.preview_cache.read();
        // TODO: not clone?
        preview_cache.get(preview_target).cloned()
    }

    pub fn insert_preview(&self, preview_target: PreviewTarget, preview: Preview) {
        let mut preview_cache = self.preview_cache.write();
        preview_cache.insert(preview_target, preview);
    }

    fn reset_scroll(&mut self) {
        self.scroll_file.take();
        self.scroll_offset = 0;
        self.current_preview_target.take();
    }

    fn prepare_scroll_file_info(
        &mut self,
        line_start: usize,
        path: PathBuf,
    ) -> Result<(ScrollFile, PathBuf)> {
        let scroll_file = match self.scroll_file {
            Some(scroll_file) => scroll_file,
            None => {
                let scroll_file = ScrollFile::new(line_start, &path)?;
                self.scroll_file.replace(scroll_file);
                scroll_file
            }
        };
        Ok((scroll_file, path))
    }

    fn set_preview_target(&mut self, preview_target: PreviewTarget) {
        self.current_preview_target.replace(preview_target);
    }

    fn scroll_preview(&mut self, direction: Direction) -> Result<PreviewTarget> {
        let new_scroll_offset = match direction {
            Direction::Up => self.scroll_offset - 1,
            Direction::Down => self.scroll_offset + 1,
        };

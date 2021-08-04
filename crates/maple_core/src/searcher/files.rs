use super::{walk_parallel, WalkConfig};
use crate::searcher::SearchContext;
use crate::stdio_server::VimProgressor;
use filter::{BestItems, MatchedItem};
use ignore::{DirEntry, WalkState};
use matcher::Matcher;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use types::ProgressUpdate;

fn search_files(
    paths: Vec<PathBuf>,
    hidden: bool,
    matcher: Matcher,
    stop_signal: Arc<AtomicBool>,
    sender: UnboundedSender<Option<MatchedItem>>,
) {
    let walk_config = WalkConfig {
        hidden,
        ..Default::default()
    };

    let search_root = paths[0].clone();

    walk_parallel(paths, walk_config).run(|| {
        let matcher = matcher.clone();
        let sender = sender.clone();
        let stop_signal = stop_signal.clone();
        let search_root = search_root.clone();
        Box::new(move |entry: Result<DirEntry, ignore::Error>| -> WalkState {
            if stop_signal.load(Ordering::SeqCst) {
                return WalkState::Quit;
            }

            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => return WalkState::Continue,
            };

            // Only search file and skip everything else.
            match entry.file_type() {
                Some(entry) if entry.is_file() => {}
                _ => return WalkState::Continue,
            };

            let path = if let Ok(p) = entry.path().s
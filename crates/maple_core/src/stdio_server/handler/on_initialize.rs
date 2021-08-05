#![allow(unused)]
use crate::process::ShellCommand;
use crate::stdio_server::job;
use crate::stdio_server::provider::{Context, ProviderSource};
use crate::tools::ctags::ProjectCtagsCommand;
use crate::tools::rg::{RgTokioCommand, RG_EXEC_CMD};
use anyhow::Result;
use filter::SourceItem;
use printer::DisplayLines;
use serde_json::{json, Value};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use types::ClapItem;
use utils::count_lines;

async fn execute_and_write_cache(
    cmd: &str,
    cache_file: std::path::PathBuf,
) -> std::io::Result<ProviderSource> {
    // Can not use subprocess::Exec::shell here.
    //
    // Must use TokioCommand otherwise the timeout may not work.

    let mut tokio_cmd = crate::process::tokio::shell_command(cmd);
    crate::process::tokio::write_stdout_to_file(&mut tokio_cmd, &cache_file).await?;
    let total = count_lines(std::fs::File::open(&cache_file)?)?;
    Ok(ProviderSource::CachedFile {
        total,
        path: cache_file,
        refreshed: true,
    })
}

/// Performs the initialization like collecting the source and total number of source items.
async fn initialize_provider_source(ctx: &Context) -> Result<ProviderSource> {
    let to_small_provider_source = |lines: Vec<String>| {
        let total = lines.len();
        let items = lines
            .into_iter()
            .map(|line| Arc::new(SourceItem::from(line)) as Arc<dyn ClapItem>)
            .collect::<Vec<_>>();
        ProviderSource::Small { total, items }
    };

    // Known providers.
    match ctx.provider_id() {
        "bli
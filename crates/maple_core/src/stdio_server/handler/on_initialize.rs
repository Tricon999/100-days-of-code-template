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
        "blines" => {
            let total = count_lines(std::fs::File::open(&ctx.env.start_buffer_path)?)?;
            let path = ctx.env.start_buffer_path.clone();
            return Ok(ProviderSource::File { total, path });
        }
        "tags" => {
            let items = crate::tools::ctags::buffer_tag_items(&ctx.env.start_buffer_path, false)?;
            let total = items.len();
            return Ok(ProviderSource::Small { total, items });
        }
        "proj_tags" => {
            let ctags_cmd = ProjectCtagsCommand::with_cwd(ctx.cwd.to_path_buf());
            let provider_source = if ctx.env.no_cache {
                let lines = ctags_cmd.execute_and_write_cache().await?;
                to_small_provider_source(lines)
            } else {
                match ctags_cmd.ctags_cache() {
                    Some((total, path)) => ProviderSource::CachedFile {
                        total,
                        path,
                        refreshed: false,
                    },
                    None => {
                        let lines = ctags_cmd.execute_and_write_cache().await?;
                        to_small_provider_source(lines)
                    }
                }
            };
            return Ok(provider_source);
        }
        "help_tags" => {
            let helplang: String = ctx.vim.eval("&helplang").await?;
            let runtimepath: String = ctx.vim.eval("&runtimepath").await?;
            let doc_tags = std::iter::once("/doc/tags".to_string()).chain(
                helplang
                    .split(',')
                    .filter(|&lang| lang != "en")
                    .map(|lang| format!("/doc/tags-{lang}")),
            );
            let lines = crate::helptags::generate_tag_lines(doc_tags, &runtimepath);
            return Ok(to_small_provider_source(lines));
        }
        _ => {}
    }

    let source_cmd: Vec<Value> = ctx.vim.bare_call("provider_source").await?;
    if let Some(value) = source_cmd.into_iter().next() {
        match value {
            // Source is a String: g:__t_string, g:__t_func_string
            Value::String(command) => {
                // Always try recreating the source.
                if ctx.provider_id() == "files" {
                    let mut tokio_cmd = crate::process::tokio::TokioCommand::new
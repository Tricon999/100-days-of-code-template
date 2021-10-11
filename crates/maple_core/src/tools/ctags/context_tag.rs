
use super::BufferTag;
use crate::tools::ctags::CTAGS_HAS_JSON_FEATURE;
use rayon::prelude::*;
use std::io::Result;
use std::ops::Deref;
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use subprocess::{Exec as SubprocessCommand, Redirection};
use tokio::process::Command as TokioCommand;
use types::ClapItem;

const CONTEXT_KINDS: &[&str] = &[
    "function",
    "method",
    "module",
    "macro",
    "implementation",
    "interface",
];

const CONTEXT_SUPERSET: &[&str] = &[
    "function",
    "method",
    "module",
    "macro",
    "implementation",
    "interface",
    "struct",
    "field",
    "typedef",
    "enumerator",
];

fn subprocess_cmd_in_json_format(file: impl AsRef<std::ffi::OsStr>) -> SubprocessCommand {
    // Redirect stderr otherwise the warning message might occur `ctags: Warning: ignoring null tag...`
    SubprocessCommand::cmd("ctags")
        .stderr(Redirection::None)
        .arg("--fields=+n")
        .arg("--output-format=json")
        .arg(file)
}

fn subprocess_cmd_in_raw_format(file: impl AsRef<std::ffi::OsStr>) -> SubprocessCommand {
    // Redirect stderr otherwise the warning message might occur `ctags: Warning: ignoring null tag...`
    SubprocessCommand::cmd("ctags")
        .stderr(Redirection::None)
        .arg("--fields=+Kn")
        .arg("-f")
        .arg("-")
        .arg(file)
}

fn tokio_cmd_in_json_format(file: &Path) -> TokioCommand {
    let mut tokio_cmd = TokioCommand::new("ctags");
    tokio_cmd
        .stderr(Stdio::null())
        .arg("--fields=+n")
        .arg("--output-format=json")
        .arg(file);
    tokio_cmd
}

fn tokio_cmd_in_raw_format(file: &Path) -> TokioCommand {
    let mut tokio_cmd = TokioCommand::new("ctags");
    tokio_cmd
        .stderr(Stdio::null())
        .arg("--fields=+Kn")
        .arg("-f")
        .arg("-")
        .arg(file);
    tokio_cmd
}

fn find_context_tag(superset_tags: Vec<BufferTag>, at: usize) -> Option<BufferTag> {
    match superset_tags.binary_search_by_key(&at, |tag| tag.line) {
        Ok(_l) => None, // Skip if the line is exactly a tag line.
        Err(_l) => {
            let context_tags = superset_tags
                .into_par_iter()
                .filter(|tag| CONTEXT_KINDS.contains(&tag.kind.as_ref()))
                .collect::<Vec<_>>();

            match context_tags.binary_search_by_key(&at, |tag| tag.line) {
                Ok(_) => None,
                Err(l) => {
                    let maybe_idx = l.checked_sub(1); // use the previous item.
                    maybe_idx.and_then(|idx| context_tags.into_iter().nth(idx))
                }
            }
        }
    }
}

/// Async version of [`current_context_tag`].
pub async fn current_context_tag_async(file: &Path, at: usize) -> Option<BufferTag> {
    let superset_tags = if *CTAGS_HAS_JSON_FEATURE.deref() {
        let cmd = tokio_cmd_in_json_format(file);
        collect_superset_context_tags_async(cmd, BufferTag::from_ctags_json, at)
            .await
            .ok()?
    } else {
        let cmd = tokio_cmd_in_raw_format(file);
        collect_superset_context_tags_async(cmd, BufferTag::from_ctags_raw, at)
            .await
            .ok()?
    };

    find_context_tag(superset_tags, at)
}

/// Returns the method/function context associated with line `at`.
pub fn current_context_tag(file: &Path, at: usize) -> Option<BufferTag> {
    let superset_tags = if *CTAGS_HAS_JSON_FEATURE.deref() {
        let cmd = subprocess_cmd_in_json_format(file);
        collect_superset_context_tags(cmd, BufferTag::from_ctags_json, at).ok()?
use crate::app::Args;
use anyhow::Result;
use clap::Parser;
use maple_core::find_usages::GtagsSearcher;
use maple_core::paths::AbsPathBuf;

/// Fuzzy filter the current vim buffer given the query.
#[derive(Parser, Debug, Clone)]
pub struct Gtags {
    /// Initial query string
    #[clap(index = 1)]
    query: String,

    /// File path of current vim buffer.
    #[clap(index = 2)]
    cwd: AbsPathBuf,

    /// Search the reference tags.
    #[clap(short, long)]
    reference: bool,
}

impl Gtags {
    pub
use crate::app::Args;
use anyhow::Result;
use clap::Parser;
use filter::SequentialSource;
use maple_core::paths::AbsPathBuf;
use matcher::{Bonus, MatchResult};
use rayon::iter::ParallelBridge;
use std::borrow::Cow;
use std::io::BufRead;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use types::ClapItem;
use utils::display_width;

/// Fuzzy filter the current vim buffer given the query.
#[derive(Parser, Debug, Clone)]
pub struct Blines {
    /// Initial query string
    #[clap(index = 1)]
    query: String,

    /// File path of current vim buffer.
    #[clap(index = 2)]
    input: AbsPathBuf,

    #[clap(long)]
    par_run: bool,
}

#[derive(Debug)]
pub struct BlinesItem {
    pub raw: String,
    pub line_number: usize,
}

impl ClapItem for BlinesItem {
    fn raw_text(&self) -> 
use crate::app::Args;
use anyhow::Result;
use clap::Parser;
use filter::{filter_sequential, FilterContext, ParallelSource, SequentialSource};
use icon::Icon;
use maple_core::paths::AbsPathBuf;
use matcher::{Bonus, FuzzyAlgorithm, MatchScope, MatcherBuilder};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use subprocess::Exec;
use types::ClapItem;
use types::MatchedItem;

fn parse_bonus(s: &str) -> Bonus {
    if s.to_lowercase().as_str() == "filename" {
        Bonus::FileName
    } else {
        Bonus::None
    }
}

/// Execute the shell command
#[derive(Parser, Debug, Clone)]
pub struct Filter {
    /// Initial query string
    #[clap(index = 1)]
    query: String,

    /// Fuzzy matching algorithm
    #[clap(long, parse(from_str), default_value = "fzy")]
    algo: FuzzyAlgorithm,

    /// Shell command to produce the whole dataset that query is applied on.
    #[clap(long)]
    cmd: Option<String>,

    /// Working directory of shell command.
    #[clap(long)]
    cmd_dir: Option<String>,

    /// Recently opened file list for adding a bonus to the initial score.
    #[clap(long, parse(from_os_str))]
    recent_files: Option<PathBuf>,

    /// Read input from a file instead of stdin, only absolute file path is supported.
    #[clap(long)]
    input: Option<AbsPathBuf>,

    /// Apply the filter on the full line content or parial of it.
    #[clap(long, parse(from_str), default_value = "full")]
    match_scope: MatchScope,

    /// Add a bonus to the score of base matching algorithm.
    #[clap(long, parse(from_str = parse_bonus), default_value = "none")]
    bonus: Bonus,

    /
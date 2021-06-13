//! Convert the source item stream to a parallel iterator and run the filtering in parallel.

use crate::{to_clap_item, FilterContext};
use anyhow::Result;
use icon::Icon;
use parking_lot::Mutex;
use printer::{println_json_with_length, DisplayLines};
use rayon::iter::{Empty, IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::cmp::Ordering as CmpOrdering;
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use subprocess::Exec;
use types::ProgressUpdate;
use types::{ClapItem, MatchedItem, Query};

/// Parallelable source.
#[derive(Debug)]
pub enum ParallelSource {
    File(PathBuf),
    Exec(Box<Exec>),
}

/// Returns the ranked results after applying fuzzy filter given the query string and a list of candidates.
///
/// Suitable for invoking the maple CLI command from shell, which will stop everything once the
/// parent is canceled.
pub fn par_dyn_run(
    query: &str,
    filter_context: FilterContext,
    par_source: Par
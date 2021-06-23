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
    par_source: ParallelSource,
) -> Result<()> {
    let query: Query = query.into();

    match par_source {
        ParallelSource::File(file) => {
            par_dyn_run_inner::<Empty<_>, _>(
                query,
                filter_context,
                ParSourceInner::Lines(std::fs::File::open(file)?),
            )?;
        }
        ParallelSource::Exec(exec) => {
            par_dyn_run_inner::<Empty<_>, _>(
                query,
                filter_context,
                ParSourceInner::Lines(exec.stream_stdout()?),
            )?;
        }
    }

    Ok(())
}

/// Generate an iterator of [`MatchedItem`] from a parallelable iterator.
pub fn par_dyn_run_list<'a, 'b: 'a>(
    query: &'a str,
    filter_context: FilterContext,
    items: impl IntoParallelIterator<Item = Arc<dyn ClapItem>> + 'b,
) {
    let query: Query = query.into();
    par_dyn_run_inner::<_, std::io::Empty>(query, filter_context, ParSourceInner::Items(items))
        .expect("Matching items in parallel can not fail");
}

#[derive(Debug)]
pub struct BestItems<P: ProgressUpdate<DisplayLines>> {
    pub icon: Icon,
    /// Time of last notification.
    pub past: Instant,
    /// Top N items.
    pub items: Vec<MatchedItem>,
    pub last_lines: Vec<String>,
    pub last_visible_highlights: Vec<Vec<usize>>,
    pub winwidth: usize,
    pub max_capacity: usize,
    pub progressor: P,
    pub update_interval: Duration,
}

impl<P: ProgressUpdate<DisplayLines>> BestItems<P> {
    pub fn new(
        icon: Icon,
        winwidth: usize,
        max_capacity: usize,
        progressor: P,
        update_interval: Duration,
    ) -> Self {
        Self {
            icon,
            past: Instant::now(),
            items: Vec::with_capacity(max_capacity),
            last_lines: Vec::with_capacity(max_capacity),
            last_visible_highlights: Vec::with_capacity(max_capacity),
            winwidth,
            max_capacity,
            progressor,
            update_interval,
        }
    }

    fn sort(&mut self) {
        self.items.sort_unstable_by(|a, b| b.cmp(a));
    }

    pub fn on_new_match(
        &mut self,
        matched_item: MatchedItem,
        total_matched: usize,
        total_processed: usize,
    ) {
        if self.items.len() < self.max_capacity {
            self.items.push(matched_item);
            self.sort();

            let now = Instant::now();
            if now > self.past + self.update_interval {
        
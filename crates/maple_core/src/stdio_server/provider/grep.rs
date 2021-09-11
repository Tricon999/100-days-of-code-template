use crate::stdio_server::provider::{ClapProvider, Context, SearcherControl};
use anyhow::Result;
use matcher::MatchScope;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use types::Query;

#[derive(Debug)]
pub struct GrepProvider {
    paths: Vec<PathBuf>,
    searcher_control: Option<SearcherControl>,
}

impl GrepProvider {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            searcher_control: None,
        }
    }

    fn process_query(&mut self, query: String, ctx: &Context) {
        if let Some(control) = self.searcher_control.take() {
            tokio::task::spawn_blocking(move || {
                control.kill();
            });
        }

        l
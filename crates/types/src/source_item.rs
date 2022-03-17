use crate::matcher::{MatchResult, Rank};
use icon::Icon;
use pattern::{extract_file_name, extract_grep_pattern, extract_tag_name};
use std::cmp::Ordering;
use std::sync::Arc;
use std::{any::Any, borrow::Cow};

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A tuple of match text piece (matching_text, offset_of_matching_text).
#[derive(Debug, Clone)]
pub struct FuzzyText<'a> {
    pub text: &'a str,
    pub matching_start: usize,
}

impl<'a> FuzzyText<'a> {
    pub fn new(text: &'a str, matching_start: usize) -> Self {
        Self {
            text,
            matching_start,
        }
    }
}

/// The location that a match should look in.
///
/// Given a query, the match scope can refer to a full string or a substring.
#[derive(Debug, Clone, Copy)]
pub enum MatchScope {
    Full,
    /// `:Clap tags`, `:Clap proj_tags`
    TagName,
    /// `:Clap files`
    FileName,
    /// `:Clap grep`
    GrepLine,
}

impl Default for MatchScope {
    fn default() -> Self {
        Self::Full
    }
}

impl std::str::FromStr for MatchScope {
    type Err = ();
    fn from_str(s
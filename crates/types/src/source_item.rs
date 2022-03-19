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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl<T: AsRef<str>> From<T> for MatchScope {
    fn from(match_scope: T) -> Self {
        match match_scope.as_ref().to_lowercase().as_str() {
            "full" => Self::Full,
            "tagname" => Self::TagName,
            "filename" => Self::FileName,
            "grepline" => Self::GrepLine,
            _ => Self::Full,
        }
    }
}

/// This trait represents the items used in the entire filter pipeline.
pub trait ClapItem: AsAny + std::fmt::Debug + Send + Sync {
    /// Initial raw text.
    fn raw_text(&self) -> &str;

    /// Text for the matching engine.
    ///
    /// Can be used to skip the leading icon, see `LineWithIcon` in `fuzzymatch-rs/src/lib.rs`.
    fn match_text(&self) -> &str {
        self.raw_text()
    }

    /// Text specifically for performing the fuzzy matching, part of the entire
    /// mathcing pipeline.
    ///
    /// The fuzzy matching process only happens when Some(_) is returned.
    fn fuzzy_text(&self, match_scope: MatchScope) -> Option<FuzzyText> {
        extract_fuzzy_text(self.match_text(), match_scope)
    }

    // TODO: Each bonus can have its own range of `bonus_text`, make use of MatchScope.
    /// Text for calculating the bonus score to tweak the initial matching score.
    fn bonus_text(&self) -> &str {
        self.match_text()
    }

    /// Callback for the result of `matcher::match_item`.
    ///
    /// Sometimes we need to tweak the indices of matched item for custom output text, e.g.,
    /// `BlinesItem`.
    fn match_result_callback(&self, match_result: MatchResult) -> MatchResult {
        match_result
    }

    /// Constructs a text intended to be displayed on the screen without any decoration (truncation,
    /// icon, etc).
    ///
    /// A concrete type of ClapItem can be structural to facilitate the matching process, in which
    /// case it's necessary to make a formatted String for displaying in the end.
    fn output_text(&self) -> Cow<'_, str> {
        self.raw_text().into()
    }

    /// Returns the icon if enabled and possible.
    fn icon(&self, icon: icon::Icon) -> Option<icon::IconType> {
        icon.icon_kind()
            .map(|icon_kind| icon_kind.icon(&self.output_text()))
    }
}

// Impl [`ClapItem`] for raw String.
//
// In order to filter/calculate bonus for a substring instead of the whole S
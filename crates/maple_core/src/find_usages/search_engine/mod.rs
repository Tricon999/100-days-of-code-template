//! Essentially, all the engines here are based on the regexp approach.
//! The difference is that `regex` engine is the poor man's way where we
//! use our own regex pattern rule with the ripgrep executable together,
//! while `ctags` and `gtags` maintain theirs which are well polished.

mod ctags;
mod gtags;
mod regex;

use super::AddressableUsage;

pub use self::ctags::CtagsSearcher;
pub use self::gtags::GtagsSearcher;
pub use self::regex::RegexSearcher;

/// When spawning the ctags/gtags request, we can specify the searching strategy.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(unused)]
pub enum QueryType {
    /// Prefix match.
    StartWith,
    /// Exact match.
    #[default]
    Exact,
    /// Substring match.
    Contain,
    ///
    Inherit,
}

/// Unified tag info.
///
/// Parsed from `ctags` and `gtags` output.
#[derive(Default, Debug)]
pub struct Symbol {
    /// None for `gtags`.
    pub name: Option<String>,
    pub path: String,
    pub pattern: String,
    pub line_number: usize,
    /// ctags only.
    pub kind: Option<String>,
    /// ctags only.
    pub scope: Option<String>,
}

impl Symbol {
    /// Parse from the output of `readtags`.
    ///
    /// TODO: add more tests
    pub fn from_readtags(s: &str) -> Option<Self> {
        let mut 
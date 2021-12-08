pub mod fzy;
pub mod skim;
pub mod substring;

use crate::MatchResult;
use types::{CaseMatching, FuzzyText};

#[derive(Debug, Clone, Copy, Default)]
pub enum FuzzyAlgorithm {
    Skim,
    #[default]
    Fzy,
}

impl std::str::FromStr for FuzzyAlgorithm {
    type Err = ();
    fn from_str(s: &str) -> 
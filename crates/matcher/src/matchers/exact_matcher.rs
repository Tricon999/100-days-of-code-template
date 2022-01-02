use crate::algo::substring::substr_indices;
use types::{CaseMatching, ExactTerm, ExactTermType, Score};

#[derive(Debug, Clone, Default)]
pub struct ExactMatcher {
    pub
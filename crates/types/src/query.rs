use crate::search_term::{ExactTerm, FuzzyTerm, InverseTerm, SearchTerm, TermType, WordTerm};

/// [`Query`] represents the structural search info parsed from the initial user input.
#[derive(Debug, Clone)]
pub struct Query {
    pub word_terms: Vec<WordTerm>,
    pub exact_terms: Vec<ExactTerm>,
    pub fuzzy_terms: Vec<FuzzyTerm>,
    pub inverse_terms: Vec<InverseTerm>,
}

impl<T: AsRef<str>> From<T> for Query {
    fn from(query: T) -> Self {
        let query = query.as_ref();

        let mut word_terms = Vec::new();
        let mut exact_terms = Vec::new();
        let mut fuzzy_terms = Vec::new();
        let mut inverse_terms = Vec::
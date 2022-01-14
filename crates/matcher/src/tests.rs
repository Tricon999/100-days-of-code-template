use super::*;
use crate::algo::fzy;
use types::SourceItem;

#[test]
fn test_resize() {
    let total_len = 100;
    let sub_query = "hello";

    let new_indices1 = {
        let mut indices = [1, 2, 3].to_vec();
        let sub_indices = (total_len - sub_query.len()..total_len).collect::<Vec<_>>();
        indices.extend_from_slice(&sub_indices);
        indices
    };

    let new_indices2 = {
        let mut indices = [1, 2, 3].to_vec();
        let mut start = total_len - sub_query.len() - 1;
        let new_len = indices.len() + sub_query.len();
        indices.resize_with(new_len, || {
            start += 1;
            start
        });
        indices
    };

    assert_eq!(new_indices1, new_indices2);
}

#[test]
fn test_match_scope_grep_line() {
    let query = "rules";
    let line = "crates/maple_cli/src/lib.rs:2:1:macro_rules! println_json {";
    let matched_item1 = fzy::fuzzy_indices(line, query, CaseMatching::Smart).unwrap();

    let item = SourceItem::from(line.to_string());
    let fuzzy_text = item.fuzzy_text(MatchScope::GrepLine).unwrap();
    let matched_item2 = FuzzyAlgorithm::Fzy
        .fuzzy_match(query, &fuzzy_text, CaseMatching::Smart)
        .unwrap();

    assert_eq!(matched_item1.indices, matched_item2.indices);
    assert!(matched_item2.score > matched_item1.score);
}

#[test]
fn test_match_scope_filename() {
    let query = "lib";
    let line = "crates/extracted_fzy/src/lib.rs";
    let matched_item1 = fzy::fuzzy_indices(line, query, CaseMatching::Smart).unwrap();

    let item = SourceItem::from(line.to_string());
    let fuzzy_text = item.fuzzy_text(MatchScope::FileName).unwrap();
    let matched_item2 = FuzzyAlgorithm::Fzy
        .fuzzy_match(query, &fuzzy_text, CaseMatching::Smart)
        .unwrap();

    assert_eq!(matched_item1.indices, matched_item2.indices);
    assert!(matched_item2.score > matched_item1.score);
}

#[test]
fn test_filename_bonus() {
    let lines = vec![
        "autoload/clap/filter.vim",
        "autoload/clap/provider/files.vim",
        "lua/fzy_filter.lua",
    ];
    let query = "fil";
    let matcher = MatcherBuilder::new()
        .bonuses(vec![Bonus::FileName])
        .build(query.into());
    for line in lines {
        let item: Arc<dyn ClapItem> = Arc::new(SourceItem::from(line.to_string()));
        let fuzzy_text = item.fuzzy_text(matcher.match_scope()).unwrap();
        let match_result_base = matcher
            .fuzzy_matcher
            .fuzzy_algo
            .fuzzy_match(query, &fuzzy_text, matcher.fuzzy_matcher.case_matching)
            .unwrap();
        let match_result_with_bonus = matcher.match_item(item).unwrap();
        assert!(match_result_base.indices == match_result_with_bonus.indices);
        assert!(match_result_with_bonus.rank[0] > match_result_base.score);
    }
}

#[test]
fn test_language_keyword_bonus() {
    let lines = vec!["hellorsr foo", "function foo"];
    let query: Query = "fo".into();
    let matcher = MatcherBuilder::new()
        .bonuses(vec![Bonus::Language("vim".into())])
        .build(query);
    let matched_item1 = matcher
        .match_item(Arc::new(lines[0]) as Arc<dyn ClapItem>)
        .unwrap();
    let matched_item2 = matcher
        .match_item(Arc::new(lines[1]) as Arc<dyn ClapItem>)
        .unwrap();
    assert!(matched_item1.indices == matched_item2.indices);
    assert!(matched_item1.rank < matched_item2.rank);
}

#[test]
fn test_exact_search_term_bonus() {
    let lines = vec!["function foo qwer", "function foo"];
    let query: Query = "'fo".into();
    let matcher = MatcherBuilder::new().build(query);
    let matched_item1 = matcher
        .match_item(Arc::new(lines[0]) as Arc<dyn ClapItem>)
        .unwrap();
    let matched_item2 = matcher
        .match_item(Arc::new(lines[1]) as Arc<dyn ClapItem>)
        .unwrap();
    asser
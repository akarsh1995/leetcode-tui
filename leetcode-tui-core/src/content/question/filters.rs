use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use leetcode_tui_db::{enums::Difficulty, DbQuestion};

use std::sync::OnceLock;

pub static FUZZY_MATCHER_CLIENT: OnceLock<SkimMatcherV2> = OnceLock::new();

pub(crate) fn filter_by_text(q: &DbQuestion, fuzzy_text: &str) -> bool {
    let search_string = format!(
        "{} {} {}", // id, topics, title
        q.id,
        q.topics
            .iter()
            .map(|t| t.slug.as_str())
            .collect::<Vec<&str>>()
            .join(", "),
        q.title
    );

    FUZZY_MATCHER_CLIENT
        .get_or_init(|| SkimMatcherV2::default())
        .fuzzy_match(search_string.as_str(), fuzzy_text)
        .is_some()
}

pub(crate) fn filter_by_difficulty(q: &DbQuestion, difficulty: &Difficulty) -> bool {
    &q.difficulty == difficulty
}

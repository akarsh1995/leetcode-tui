use std::collections::HashMap;

use crate::{app_ui::helpers::utils::slugify, entities::TopicTagModel};

use once_cell::sync::Lazy;

pub(crate) struct CustomQuestionList<'a> {
    title: &'a str,
    slugs: &'a [&'a str],
}

impl<'a> CustomQuestionList<'a> {
    fn new(slugs: &'a [&'a str], title: &'a str) -> Self {
        Self { title, slugs }
    }
}

impl CustomQuestionList<'_> {
    fn get_id(&self) -> String {
        slugify(self.title)
    }

    pub(crate) fn get_topic_tag(&self) -> TopicTagModel {
        let id = self.get_id();
        TopicTagModel {
            name: self.title.to_string(),
            id: id.clone(),
            slug: id.clone(),
        }
    }

    fn get_slug_index_map(&self) -> HashMap<&str, usize> {
        self.slugs
            .iter()
            .enumerate()
            .map(|(index, slug)| (*slug, index))
            .collect::<HashMap<_, _>>()
    }

    pub(super) fn filter_questions<'b, QS: Iterator<Item = &'b super::Question>>(
        &self,
        questions: QS,
    ) -> Vec<super::Question> {
        let map = self.get_slug_index_map();
        let mut filtered_questions = vec![None; self.slugs.len()];
        for question in questions {
            let slug = &question.borrow().title_slug;
            if let Some(index) = map.get(slug.as_str()).copied() {
                filtered_questions[index].replace(question.clone());
            }
        }
        map.iter()
            .zip(filtered_questions)
            .map(|((slug, _), maybe_model)| {
                maybe_model.unwrap_or_else(|| {
                    panic!(
                        "Question slug {} does no match with any question in the database.",
                        *slug
                    )
                })
            })
            .collect()
    }
}

const _NEETCODE_75: [&str; 75] = [
    "contains-duplicate",
    "valid-anagram",
    "two-sum",
    "group-anagrams",
    "top-k-frequent-elements",
    "product-of-array-except-self",
    "encode-and-decode-strings",
    "longest-consecutive-sequence",
    "valid-palindrome",
    "3sum",
    "container-with-most-water",
    "best-time-to-buy-and-sell-stock",
    "longest-substring-without-repeating-characters",
    "longest-repeating-character-replacement",
    "minimum-window-substring",
    "valid-parentheses",
    "find-minimum-in-rotated-sorted-array",
    "search-in-rotated-sorted-array",
    "reverse-linked-list",
    "merge-two-sorted-lists",
    "reorder-list",
    "remove-nth-node-from-end-of-list",
    "linked-list-cycle",
    "merge-k-sorted-lists",
    "invert-binary-tree",
    "maximum-depth-of-binary-tree",
    "same-tree",
    "subtree-of-another-tree",
    "lowest-common-ancestor-of-a-binary-search-tree",
    "binary-tree-level-order-traversal",
    "validate-binary-search-tree",
    "kth-smallest-element-in-a-bst",
    "construct-binary-tree-from-preorder-and-inorder-traversal",
    "binary-tree-maximum-path-sum",
    "serialize-and-deserialize-binary-tree",
    "implement-trie-prefix-tree",
    "design-add-and-search-words-data-structure",
    "word-search-ii",
    "find-median-from-data-stream",
    "combination-sum",
    "word-search",
    "number-of-islands",
    "clone-graph",
    "pacific-atlantic-water-flow",
    "course-schedule",
    "number-of-connected-components-in-an-undirected-graph",
    "graph-valid-tree",
    "alien-dictionary",
    "climbing-stairs",
    "house-robber",
    "house-robber-ii",
    "longest-palindromic-substring",
    "palindromic-substrings",
    "decode-ways",
    "coin-change",
    "maximum-product-subarray",
    "word-break",
    "longest-increasing-subsequence",
    "unique-paths",
    "longest-common-subsequence",
    "maximum-subarray",
    "jump-game",
    "insert-interval",
    "merge-intervals",
    "non-overlapping-intervals",
    "meeting-rooms",
    "meeting-rooms-ii",
    "rotate-image",
    "spiral-matrix",
    "set-matrix-zeroes",
    "number-of-1-bits",
    "counting-bits",
    "reverse-bits",
    "missing-number",
    "sum-of-two-integers",
];

pub(crate) static NEETCODE_75: Lazy<CustomQuestionList<'static>> =
    Lazy::new(|| CustomQuestionList::new(_NEETCODE_75.as_slice(), "Neetcode 75"));

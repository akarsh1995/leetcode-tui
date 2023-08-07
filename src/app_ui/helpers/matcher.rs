use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub(crate) struct Matcher<'a> {
    values: &'a [&'a str],
}

impl Matcher<'_> {
    pub(crate) fn match_with_key(&self, needle: &str) -> Vec<usize> {
        let matcher = SkimMatcherV2::default();
        self.values
            .iter()
            .enumerate()
            .filter_map(|(idx, haystack)| matcher.fuzzy_match(haystack, needle).map(|_| idx))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let m = Matcher {
            values: &["bello", "helloworld"],
        };
        assert_eq!(m.match_with_key("bello"), vec![0]);
        assert_eq!(m.match_with_key("elo"), vec![0, 1]);
        assert_eq!(m.match_with_key("hello"), vec![1]);
    }
}

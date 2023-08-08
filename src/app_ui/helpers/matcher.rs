use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub(crate) struct Matcher<'a, T: Iterator<Item = &'a str>> {
    values: Option<T>,
}

impl<'a, T: Iterator<Item = &'a str>> Matcher<'a, T> {
    pub fn new(values: Option<T>) -> Self {
        Self { values }
    }

    pub(crate) fn match_with_key(&mut self, needle: &str) -> Option<Vec<usize>> {
        let matcher = SkimMatcherV2::default();
        let mut res = vec![];
        if let Some(iterator) = self.values.take() {
            for (i, haystack) in iterator.enumerate() {
                if matcher.fuzzy_match(haystack, needle).is_some() {
                    res.push(i);
                }
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let values = vec!["bello", "helloworld"];
        let values2 = values.clone();
        let values3 = values.clone();
        let mut m = Matcher {
            values: Some(values.into_iter()),
        };
        assert_eq!(m.match_with_key("bello"), Some(vec![0]));

        let mut m = Matcher {
            values: Some(values2.into_iter()),
        };

        assert_eq!(m.match_with_key("elo"), Some(vec![0, 1]));

        let mut m = Matcher {
            values: Some(values3.into_iter()),
        };

        assert_eq!(m.match_with_key("hello"), Some(vec![1]));
    }
}

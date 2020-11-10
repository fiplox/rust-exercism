use std::collections::HashSet;
use std::iter::FromIterator;

fn normalize(word: &str) -> String {
    let mut w: Vec<_> = word.to_uppercase().chars().collect();
    w.sort();
    w.iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    HashSet::from_iter(
        possible_anagrams
            .iter()
            .cloned()
            .filter(|x| normalize(x) == normalize(word) && x.to_uppercase() != word.to_uppercase()),
    )
}

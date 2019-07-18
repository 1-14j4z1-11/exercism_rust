use std::collections::HashSet;
use std::iter::Iterator;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for w in possible_anagrams {
        if are_anagram(word, w) {
            set.insert(*w);
        }
    }

    set
}

fn are_anagram(word1: &str, word2: &str) -> bool {
    if word1.to_lowercase() == word2.to_lowercase() {
        return false;
    }

    let mut chars1 = word1.to_lowercase().chars().collect::<Vec<_>>();
    let mut chars2 = word2.to_lowercase().chars().collect::<Vec<_>>();

    chars1.sort_by(|x, y| x.cmp(y));
    chars2.sort_by(|x, y| x.cmp(y));

    chars1 == chars2
}

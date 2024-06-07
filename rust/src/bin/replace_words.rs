// https://leetcode.com/problems/replace-words/
// 2024/06/07

use std::collections::HashSet;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let dictionary: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();

        sentence
            .split(' ')
            .map(|s| {
                for i in 0..s.len() {
                    if dictionary.contains(&s[0..i]) {
                        return &s[0..i];
                    }
                }

                return s;
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::replace_words(
            ["cat", "bat", "rat"].map(|s| s.to_string()).to_vec(),
            "the cattle was rattled by the battery".to_string()
        )
    ); // "the cat was rat by the bat"

    println!(
        "{:?}",
        Solution::replace_words(
            ["a", "b", "c"].map(|s| s.to_string()).to_vec(),
            "aadsfasf absbs bbab cadsfafs".to_string()
        )
    ); // "a a b c"
}

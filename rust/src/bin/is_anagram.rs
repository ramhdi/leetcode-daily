// https://leetcode.com/problems/valid-anagram/
// 2023/12/16

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut first_chars: HashMap<char, usize> = HashMap::new();
        let mut second_chars: HashMap<char, usize> = HashMap::new();

        for c1 in s.chars() {
            let counter = first_chars.entry(c1).or_insert(0);
            *counter += 1;
        }

        for c2 in t.chars() {
            let counter = second_chars.entry(c2).or_insert(0);
            *counter += 1;
        }

        for (key, value) in first_chars {
            if let Some(&v) = second_chars.get(&key) {
                if value != v {
                    return false;
                }
            } else {
                return false;
            }
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagatam".to_string())
    );
    println!(
        "{}",
        Solution::is_anagram("kjfewr".to_string(), "dslk".to_string())
    );
}

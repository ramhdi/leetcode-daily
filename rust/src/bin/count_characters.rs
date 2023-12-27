// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/

use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut result: i32 = 0;
        let mut chars_map: HashMap<char, i32> = HashMap::new();
        for c in chars.chars() {
            *chars_map.entry(c).or_insert(0) += 1;
        }

        for word in words {
            let mut chars_map2 = chars_map.clone();
            let mut i: usize = 0;
            for c in word.chars() {
                *chars_map2.entry(c).or_insert(0) -= 1;
                if chars_map2.get(&c).unwrap() < &0 {
                    break;
                } else {
                    i += 1;
                }
            }
            if i == word.len() {
                result += i as i32;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_characters(
            ["cat", "bt", "hat", "tree"].map(|s| s.to_string()).to_vec(),
            "atach".to_string(),
        )
    );
    println!(
        "{:?}",
        Solution::count_characters(
            ["hello", "world", "leetcode"]
                .map(|s| s.to_string())
                .to_vec(),
            "welldonehoneyr".to_string(),
        )
    );
}

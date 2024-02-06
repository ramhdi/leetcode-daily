// https://leetcode.com/problems/group-anagrams/
// 2024/02/06

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut sorted_chars: Vec<char> = s.chars().collect();
            sorted_chars.sort_unstable();

            let sorted_str: String = sorted_chars.iter().collect();

            anagram_map.entry(sorted_str).or_insert(Vec::new()).push(s);
        }

        return anagram_map.into_values().collect();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(
            ["eat", "tea", "tan", "ate", "nat", "bat"]
                .map(|s| s.to_string())
                .to_vec()
        )
    ); // [["bat"],["nat","tan"],["ate","eat","tea"]]

    println!(
        "{:?}",
        Solution::group_anagrams([""].map(|s| s.to_string()).to_vec())
    ); // [[""]]

    println!(
        "{:?}",
        Solution::group_anagrams(["a"].map(|s| s.to_string()).to_vec())
    ); // [["a"]]
}

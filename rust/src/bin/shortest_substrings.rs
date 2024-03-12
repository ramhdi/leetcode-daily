// https://leetcode.com/contest/weekly-contest-388/problems/shortest-uncommon-substring-in-an-array/

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        fn is_uncommon(substring: &str, arr: &[String], index: usize) -> bool {
            for (i, s) in arr.iter().enumerate() {
                if i == index {
                    continue; // Skip the current string
                }
                if s.contains(substring) {
                    return false; // Substring is found in another string
                }
            }
            true // Substring is uncommon
        }

        let mut result = Vec::new();
        for (i, s) in arr.iter().enumerate() {
            let mut shortest_uncommon = String::new();
            for start in 0..s.len() {
                for end in start + 1..=s.len() {
                    let substring = &s[start..end];
                    if is_uncommon(substring, &arr, i) {
                        if shortest_uncommon.is_empty() || substring.len() < shortest_uncommon.len()
                        {
                            shortest_uncommon = String::from(substring);
                        } else if substring.len() == shortest_uncommon.len()
                            && substring < &shortest_uncommon
                        {
                            shortest_uncommon = String::from(substring);
                        }
                    }
                }
            }
            result.push(shortest_uncommon);
        }
        result
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_substrings(["cab", "ad", "bad", "c"].map(|s| s.to_string()).to_vec())
    ); // ["ab","","ba",""]

    println!(
        "{:?}",
        Solution::shortest_substrings(["abc", "bcd", "abcd"].map(|s| s.to_string()).to_vec())
    ); // ["","","abcd"]
}

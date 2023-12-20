// https://leetcode.com/problems/merge-strings-alternately
// LeetCode 75

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result: String = String::new();
        let n = std::cmp::max(word1.len(), word2.len());

        for i in 0..n {
            if let Some(c1) = word1.chars().nth(i) {
                result.push(c1);
            }

            if let Some(c2) = word2.chars().nth(i) {
                result.push(c2);
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::merge_alternately("ab".to_string(), "pqrs".to_string())
    );
}

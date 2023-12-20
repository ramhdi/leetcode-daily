// https://leetcode.com/problems/jewels-and-stones/

use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut result: i32 = 0;

        let mut jewels_set: HashSet<char> = HashSet::new();
        for j in jewels.chars() {
            jewels_set.insert(j);
        }

        for s in stones.chars() {
            if jewels.contains(s) {
                result += 1;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
    );
}

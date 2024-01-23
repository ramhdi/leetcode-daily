// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
// 2024/01/23

use std::collections::HashSet;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut result = 0;
        Solution::dfs(&arr, String::new(), 0, &mut result);
        return result;
    }

    fn dfs(arr: &Vec<String>, path: String, idx: usize, result: &mut i32) {
        if !Solution::is_unique_chars(&path) {
            return;
        }

        *result = (*result).max(path.len() as i32);

        if idx == arr.len() {
            return;
        }

        for i in idx..arr.len() {
            let new_path = path.clone() + &arr[i];
            Solution::dfs(arr, new_path, i + 1, result);
        }
    }

    fn is_unique_chars(s: &str) -> bool {
        let mut set = HashSet::new();
        for c in s.chars() {
            if set.contains(&c) {
                return false;
            }
            set.insert(c);
        }
        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_length(["un", "iq", "ue"].map(|s| s.to_string()).to_vec())
    ); // 4
    println!(
        "{:?}",
        Solution::max_length(["cha", "r", "act", "ers"].map(|s| s.to_string()).to_vec())
    ); // 6
    println!(
        "{:?}",
        Solution::max_length(
            ["abcdefghijklmnopqrstuvwxyz"]
                .map(|s| s.to_string())
                .to_vec()
        )
    ); // 26
}

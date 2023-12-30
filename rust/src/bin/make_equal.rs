// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
// 2023/12/30

use std::collections::HashMap;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        if n == 1 {
            return true;
        }
        let mut count: Vec<usize> = vec![0; 26];
        for word in words {
            for ch in word.chars() {
                count[ch as usize - 'a' as usize] += 1;
            }
        }

        for c in count {
            if c % n != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn _make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        if n == 1 {
            return true;
        }
        let mut count: HashMap<char, usize> = HashMap::new();
        for word in words {
            for c in word.chars() {
                *count.entry(c).or_insert(0) += 1;
            }
        }

        for (_, v) in count {
            if v % n != 0 {
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
        Solution::make_equal(["abc", "aabc", "bc"].map(|s| s.to_string()).to_vec())
    );
    println!(
        "{}",
        Solution::make_equal(["ab", "a"].map(|s| s.to_string()).to_vec())
    );
}

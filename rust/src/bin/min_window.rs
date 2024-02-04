// https://leetcode.com/problems/minimum-window-substring/
// 2024/02/04

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut state = [0 as i32; (b'z' - b'A' + 1) as usize];

        let mut target = [0 as i32; (b'z' - b'A' + 1) as usize];
        for b in t.bytes() {
            target[(b - b'A') as usize] += 1;
        }

        let string = s.as_bytes();

        let mut best_from: usize = 0;
        let mut best_to: usize = 0;

        let mut from: usize = 0;
        let mut to: usize = 0;

        let chars_expected = target.iter().filter(|&&ch| ch > 0).count();
        let mut chars_found = 0;

        while to < string.len() {
            let ch = (string[to] - b'A') as usize;
            if target[ch] > 0 {
                if state[ch] + 1 == target[ch] {
                    chars_found += 1;
                }
                state[ch] += 1;
            }

            if chars_found == chars_expected {
                let mut fch = (string[from] - b'A') as usize;
                while target[fch] == 0 || state[fch] > target[fch] {
                    if state[fch] > 0 {
                        state[fch] -= 1;
                    }

                    from += 1;
                    fch = (string[from] - b'A') as usize;
                }

                if to - from < best_to - best_from || best_to == 0 {
                    best_from = from;
                    best_to = to + 1;
                }
            }

            to += 1;
        }

        return s[best_from..best_to].to_string();
    }

    pub fn _min_window(s: String, t: String) -> String {
        let mut result: String = String::new();
        let m = s.len();

        let mut count_t: HashMap<char, usize> = HashMap::new();
        for c in t.chars() {
            *count_t.entry(c).or_insert(0) += 1;
        }

        let mut start: usize = 0;
        let mut end: usize = 1;

        while end <= m && start < end {
            let slice = &s[start..end];
            let mut includes: bool = true;

            let mut count_slice: HashMap<char, usize> = HashMap::new();
            for c in slice.chars() {
                *count_slice.entry(c).or_insert(0) += 1;
            }

            for (k, v) in count_t.clone() {
                if &v > count_slice.get(&k).unwrap_or(&0) {
                    includes = false;
                    break;
                }
            }

            if !includes {
                end += 1;
            } else {
                if result == "".to_string() || slice.len() < result.len() {
                    result = slice.to_string();
                }
                start += 1;
            }
        }

        return result;
    }
}
pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    ); // "BANC"

    println!(
        "{:?}",
        Solution::min_window("a".to_string(), "a".to_string())
    ); // "a"

    println!(
        "{:?}",
        Solution::min_window("a".to_string(), "aa".to_string())
    ); // ""
}

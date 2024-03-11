// https://leetcode.com/problems/custom-sort-string/
// 2024/03/11

use std::collections::HashSet;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut result: String = String::new();
        let mut count: [u8; 26] = [0; (b'z' - b'a' + 1) as usize];
        let order_set: HashSet<char> = order.chars().collect();

        for &c in s.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        for c in order.chars() {
            for _ in 0..count[c as usize - b'a' as usize] {
                result.push(c);
            }
        }

        for c in s.chars() {
            if !order_set.contains(&c) {
                result.push(c);
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::custom_sort_string("cba".to_string(), "abcd".to_string())
    ); // "cbad"

    println!(
        "{:?}",
        Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string())
    ); // "bcad"
}

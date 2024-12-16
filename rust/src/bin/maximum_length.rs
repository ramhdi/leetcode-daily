// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/
// 2024/12/10

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s: &str = &s;

        (0..s.len())
            .flat_map(|start| (start + 1..=s.len()).map(move |end| &s[start..end]))
            .filter(|substring| {
                let mut chars = substring.chars();
                let first_char = chars.next().unwrap();
                chars.all(|c| c == first_char)
            })
            .fold(HashMap::<&str, usize>::new(), |mut counts, substring| {
                *counts.entry(substring).or_insert(0) += 1;
                counts
            })
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(substring, _)| substring.len() as i32)
            .max()
            .unwrap_or(-1)
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::maximum_length("aaaa".to_string())); // 2

    println!("{:?}", Solution::maximum_length("abcdef".to_string())); // -1

    println!("{:?}", Solution::maximum_length("abcaba".to_string())); // 1

    println!(
        "{:?}",
        Solution::maximum_length("cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde".to_string())
    ); // 2
}

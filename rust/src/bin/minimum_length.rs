// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/
// 2024/03/04

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut s = s.as_str();
        while s.len() > 1 && *s.as_bytes().first().unwrap() == *s.as_bytes().last().unwrap() {
            let c = s.chars().next().unwrap();
            s = s.trim_matches(c);
        }
        s.len() as i32
    }
}
pub struct Solution {}

fn main() {
    println!("{:?}", Solution::minimum_length("ca".to_string())); // 2

    println!("{:?}", Solution::minimum_length("cabaabac".to_string())); // 0

    println!("{:?}", Solution::minimum_length("aabccabba".to_string())); // 3
}

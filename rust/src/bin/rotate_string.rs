// https://leetcode.com/problems/rotate-string/
// 2024/11/03

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.repeat(2).contains(&goal)
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string())
    ); // true

    println!(
        "{:?}",
        Solution::rotate_string("abcde".to_string(), "abced".to_string())
    ); // false
}

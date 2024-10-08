// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
// 2024/10/07

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            if let Some(&prev_char) = stack.last() {
                if (prev_char == 'A' && c == 'B') || (prev_char == 'C' && c == 'D') {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
        stack.len() as i32
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_length("ABFCACDB".to_string())); // 2

    println!("{:?}", Solution::min_length("ACBBD".to_string())); // 5
}

// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/
// 2024/09/03

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut result: i32 = 0;

        // Convert
        let mut converted = s
            .bytes()
            .fold(String::new(), |acc, c| acc + &(c - b'a' + 1).to_string());

        // Transform
        for _ in 0..k {
            result = converted.bytes().fold(0, |acc, c| acc + (c - b'0') as i32);
            converted = result.to_string();
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::get_lucky("iiii".to_string(), 1)); // 36

    println!("{}", Solution::get_lucky("leetcode".to_string(), 2)); // 6
}

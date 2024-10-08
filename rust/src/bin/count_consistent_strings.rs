// https://leetcode.com/problems/count-the-number-of-consistent-strings/
// 2024/09/12

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut result: i32 = 0;
        let allowed = allowed
            .bytes()
            .fold(0, |acc, c| acc | 1 << (c as u8 - b'a'));

        for word in words {
            result += 1;
            for c in word.bytes() {
                if allowed != (allowed | 1 << (c as u8 - b'a')) {
                    result -= 1;
                    break;
                }
            }
        }

        result
    }
}
pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_consistent_strings(
            "ab".to_string(),
            ["ad", "bd", "aaab", "baa", "badab"]
                .map(|e| e.to_string())
                .to_vec()
        )
    ); // 2
}

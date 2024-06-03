// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/
// 2024/06/03

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut j: usize = 0;

        for &c in s.iter() {
            if j < t.len() && c == t[j] {
                j += 1;
            }
        }

        return (t.len() - j) as i32;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::append_characters("coaching".to_string(), "coding".to_string())
    ); // 4

    println!(
        "{:?}",
        Solution::append_characters("abcde".to_string(), "a".to_string())
    ); // 0

    println!(
        "{:?}",
        Solution::append_characters("z".to_string(), "abcde".to_string())
    ); // 5
}

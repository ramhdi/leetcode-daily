// https://leetcode.com/problems/string-compression-iii/
// 2024/11/04

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut comp = String::new();
        let chars = word.as_bytes();
        let n = chars.len();
        let mut i = 0;

        while i < n {
            let current_char = chars[i];
            let mut count = 1;

            while i + count < n && chars[i + count] == chars[i] && count < 9 {
                count += 1;
            }

            comp.push((count as u8 + 48) as char);
            comp.push(current_char as char);

            i += count;
        }

        comp
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::compressed_string("abcde".to_string())); // "1a1b1c1d1e"

    println!(
        "{:?}",
        Solution::compressed_string("aaaaaaaaaaaaaabb".to_string())
    ); // "9a5a2b"
}

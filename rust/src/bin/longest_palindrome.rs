// https://leetcode.com/problems/longest-palindrome/
// 2024/06/04

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut result: i32 = 0;
        const N: usize = (b'z' - b'A') as usize + 1;
        let mut char_count: [i32; N] = [0; N];

        for c in s.bytes() {
            char_count[(c - b'A') as usize] += 1;
        }

        for c in char_count {
            result = (result + (c & !1)) | c & 1;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::longest_palindrome("abccccdd".to_string())); // 7

    println!("{:?}", Solution::longest_palindrome("a".to_string())); // 1

    println!("{:?}", Solution::longest_palindrome("bananas".to_string())); // 5

    println!(
        "{:?}",
        Solution::longest_palindrome(
            "zeusnilemacaronimaisanitratetartinasiaminoracamelinsuez".to_string()
        )
    ); // 5
}

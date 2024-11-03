// https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// 2024/11/01

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::new();
        let (mut last_char, mut last_char_count) = (' ', 0);

        for c in s.chars() {
            if c == last_char {
                last_char_count += 1;
            } else {
                last_char = c;
                last_char_count = 1;
            }

            if last_char_count < 3 {
                result.push(c);
            }
        }

        result
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::make_fancy_string("leeetcode".to_string())); // "leetcode"

    println!("{:?}", Solution::make_fancy_string("aaabaaaa".to_string())); // "aabaa"

    println!("{:?}", Solution::make_fancy_string("aab".to_string())); // "aab"
}

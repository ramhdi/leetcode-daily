// https://leetcode.com/problems/length-of-last-word/
// 2024/04/01

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut space_before = false;

        for c in s.chars() {
            if c != ' ' {
                if space_before {
                    result = 0;
                    space_before = false;
                }
                result += 1;
            } else {
                space_before = true;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::length_of_last_word("Hello World".to_string())
    ); // 5

    println!(
        "{:?}",
        Solution::length_of_last_word("   fly me   to   the moon  ".to_string())
    ); // 4

    println!(
        "{:?}",
        Solution::length_of_last_word("luffy is still joyboy".to_string())
    ); // 6
}

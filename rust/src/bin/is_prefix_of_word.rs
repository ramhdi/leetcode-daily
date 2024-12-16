// https://leetcode.com/problems/valid-arrangement-of-pairs/
// 2024/12/02

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .enumerate()
            .find(|(_, s)| s.starts_with(&search_word))
            .map_or(-1, |(i, _)| i as i32 + 1)
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string())
    ); // 4

    println!(
        "{:?}",
        Solution::is_prefix_of_word(
            "this problem is an easy problem".to_string(),
            "pro".to_string()
        )
    ); // 2

    println!(
        "{:?}",
        Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string())
    ); // -1
}

// https://leetcode.com/problems/valid-arrangement-of-pairs/
// 2024/12/02

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::with_capacity(2 * s.len());
        let mut j = 0;
        for (i, c) in s.chars().enumerate() {
            if j < spaces.len() && i == spaces[j] as usize {
                result.push(' ');
                j += 1;
            }
            result.push(c);
        }
        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), [8, 13, 15].to_vec())
    ); // "Leetcode Helps Me Learn"

    println!(
        "{:?}",
        Solution::add_spaces("icodeinpython".to_string(), [1, 5, 7, 9].to_vec())
    ); // "i code in py thon"

    println!(
        "{:?}",
        Solution::add_spaces("spacing".to_string(), [0, 1, 2, 3, 4, 5, 6].to_vec())
    ); // " s p a c i n g"
}

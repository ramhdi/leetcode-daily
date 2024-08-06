// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/
// 2024/08/06

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut char_count = vec![0usize; 26];
        let mut cost = 0;

        for c in word.chars() {
            char_count[c as usize - 'a' as usize] += 1;
        }

        char_count.sort_unstable();

        for (i, &v) in char_count.iter().rev().enumerate() {
            cost += (i / 8 + 1) * v;
        }

        cost as i32
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::minimum_pushes("abcde".to_string())); // 5

    println!("{}", Solution::minimum_pushes("xyzxyzxyzxyz".to_string())); // 12

    println!(
        "{}",
        Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string())
    ); // 24
}

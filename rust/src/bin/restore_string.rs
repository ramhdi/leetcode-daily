// https://leetcode.com/problems/shuffle-string/

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result: Vec<char> = vec![0 as char; indices.len()];

        for (c, i) in s.chars().zip(indices) {
            result[i as usize] = c;
        }
        return result.into_iter().collect();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3])
    );
}

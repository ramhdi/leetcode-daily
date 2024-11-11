// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/
// 2024/11/07

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut result = 0;

        for b in 0..24 {
            let mut current_set = 0;
            for i in 0..candidates.len() {
                if candidates[i] & (1 << b) > 0 {
                    current_set += 1;
                }
            }

            result = result.max(current_set);
        }

        result
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::largest_combination([16, 17, 71, 62, 12, 24, 14].to_vec())
    ); // 4

    println!("{:?}", Solution::largest_combination([8, 8].to_vec())); // 2
}

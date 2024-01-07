// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// 2024/01/07

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result: i32 = 0;
        let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];

        for i in 1..n {
            for j in 0..i {
                let diff_long = nums[i] as i64 - nums[j] as i64;
                if !(diff_long > i32::MAX as i64 || diff_long < i32::MIN as i64) {
                    let diff = diff_long as i32;
                    *dp[i].entry(diff).or_insert(0) += 1;

                    if let Some(&cnt) = dp[j].get(&diff) {
                        *dp[i].entry(diff).or_insert(0) += cnt;
                        result += cnt;
                    }
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
    ); // 7
    println!(
        "{:?}",
        Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7])
    ); // 16
    println!(
        "{:?}",
        Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296])
    ); // 0
}

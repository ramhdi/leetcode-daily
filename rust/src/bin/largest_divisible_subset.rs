// https://leetcode.com/problems/largest-divisible-subset/
// 2024/02/09

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev = vec![0; n];
        let mut max_len = 0;
        let mut max_idx = 0;

        nums.sort();

        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                max_idx = i;
            }
        }

        for _ in 0..max_len {
            result.push(nums[max_idx]);
            max_idx = prev[max_idx];
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::largest_divisible_subset([1, 2, 3].to_vec())
    ); // [1,2], [1,3]
    println!(
        "{:?}",
        Solution::largest_divisible_subset([1, 2, 4, 8].to_vec())
    ); // [1,2,4,8]
}

// https://leetcode.com/problems/partition-array-for-maximum-sum/
// 2024/02/03

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            let mut max_val = arr[i - 1];
            for j in 1..=(std::cmp::min(i, k as usize)) {
                max_val = std::cmp::max(max_val, arr[i - j]);
                dp[i] = std::cmp::max(dp[i], dp[i - j] + max_val * j as i32);
            }
        }

        return dp[n];
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_sum_after_partitioning([1, 15, 7, 9, 2, 5, 10].to_vec(), 3)
    ); // 84

    println!(
        "{:?}",
        Solution::max_sum_after_partitioning([1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3].to_vec(), 4)
    ); // 83

    println!(
        "{:?}",
        Solution::max_sum_after_partitioning([1].to_vec(), 1)
    ); // 1
}

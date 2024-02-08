// https://leetcode.com/problems/perfect-squares/
// 2024/02/08

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }

        return dp[n];
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::num_squares(12)); // 3
    println!("{:?}", Solution::num_squares(13)); // 2
}

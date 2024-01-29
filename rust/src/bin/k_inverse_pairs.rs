// https://leetcode.com/problems/k-inverse-pairs-array/
// 2024/01/27

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let mut dp = vec![vec![0; (k + 1) as usize]; (n + 1) as usize];

        for i in 1..=n {
            dp[i as usize][0] = 1;
        }

        for i in 1..=n {
            for j in 1..=k {
                let mut sum = dp[i as usize][j as usize - 1];
                if j >= i {
                    sum = (sum + MOD - dp[(i - 1) as usize][(j - i) as usize]) % MOD;
                }
                dp[i as usize][j as usize] = (dp[i as usize - 1][j as usize] + sum) % MOD;
            }
        }

        return (dp[n as usize][k as usize] + MOD
            - if k > 0 {
                dp[n as usize][k as usize - 1]
            } else {
                0
            })
            % MOD;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::k_inverse_pairs(3, 0)); // 1
    println!("{:?}", Solution::k_inverse_pairs(3, 1)); // 2
}

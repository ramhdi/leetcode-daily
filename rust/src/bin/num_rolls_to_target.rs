// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/
// 2023/12/26

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; target as usize + 1]; n as usize + 1];

        fn dp_helper(n: usize, k: usize, target: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
            if n == 0 && target == 0 {
                return 1;
            }
            if n == 0 || target <= 0 {
                return 0;
            }

            if dp[n][target as usize] != -1 {
                return dp[n][target as usize] % MOD;
            }

            let mut result: i32 = 0;
            for face in 1..=k {
                result = (result + dp_helper(n - 1, k, target - face as i32, dp)) % MOD
            }

            dp[n][target as usize] = result % MOD;
            return result;
        }

        return dp_helper(n as usize, k as usize, target, &mut dp);
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::num_rolls_to_target(1, 6, 3));
    println!("{:?}", Solution::num_rolls_to_target(2, 6, 7));
    println!("{:?}", Solution::num_rolls_to_target(30, 30, 500));
}

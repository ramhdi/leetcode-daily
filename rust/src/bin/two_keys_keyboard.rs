// https://leetcode.com/problems/2-keys-keyboard/
// 2024/08/19

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![std::i32::MAX; (n + 1) as usize];
        dp[0] = 0;
        dp[1] = 0;

        for i in 2..=n {
            for j in 1..=i / 2 {
                if i % j == 0 {
                    dp[i as usize] = dp[i as usize].min(dp[j as usize] + i / j);
                }
            }
        }

        dp[n as usize]
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::min_steps(3)); // 3

    println!("{:?}", Solution::min_steps(1)); // 0
}

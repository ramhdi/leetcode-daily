// https://leetcode.com/problems/strange-printer/
// 2024/08/21

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }

        for length in 2..=n {
            for i in 0..=(n - length) {
                let j = i + length - 1;
                dp[i][j] = dp[i][j - 1] + 1;

                for k in i..j {
                    if s[k] == s[j] {
                        dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k + 1][j - 1]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::strange_printer("aaabbb".to_string())); // 2
    println!("{}", Solution::strange_printer("aba".to_string())); // 2
    println!("{}", Solution::strange_printer("abc".to_string())); // 3
}

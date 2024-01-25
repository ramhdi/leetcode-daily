// https://leetcode.com/problems/longest-common-subsequence/
// 2024/01/25

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let len1 = text1.len();
        let len2 = text2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        return dp[len1][len2];
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    ); // 3

    println!(
        "{:?}",
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
    ); // 3

    println!(
        "{:?}",
        Solution::longest_common_subsequence("abc".to_string(), "def".to_string())
    ); // 0
}

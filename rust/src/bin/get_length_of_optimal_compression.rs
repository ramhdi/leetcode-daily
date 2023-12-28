// https://leetcode.com/problems/string-compression-ii/

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let n: usize = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![9999; 110]; 110];
        dp[0][0] = 0;

        for i in 1..=n {
            for j in 0..=k as usize {
                let mut cnt: usize = 0;
                let mut del: usize = 0;
                for l in (1..=i).rev() {
                    if s.chars().nth(l - 1).unwrap() == s.chars().nth(i - 1).unwrap() {
                        cnt += 1;
                    } else {
                        del += 1;
                    }

                    if j >= del {
                        dp[i][j] = dp[i][j].min(
                            dp[l - 1][j - del]
                                + 1
                                + if cnt >= 100 {
                                    3
                                } else if cnt >= 10 {
                                    2
                                } else if cnt >= 2 {
                                    1
                                } else {
                                    0
                                },
                        );
                    }
                }

                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }
            }
        }
        return dp[n][k as usize];
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2)
    );
    println!(
        "{:?}",
        Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2)
    );
    println!(
        "{:?}",
        Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0)
    );
}

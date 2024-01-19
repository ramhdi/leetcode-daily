// https://leetcode.com/problems/minimum-falling-path-sum/
// 2024/01/19

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<i32> = matrix[0].clone();
        let n = matrix[0].len();

        for i in 1..n {
            let mut temp: Vec<i32> = vec![0; n];
            for j in 0..n {
                if j == 0 {
                    temp[j] = matrix[i][j] + std::cmp::min(dp[j], dp[j + 1]);
                } else if j == n - 1 {
                    temp[j] = matrix[i][j] + std::cmp::min(dp[j], dp[j - 1]);
                } else {
                    temp[j] =
                        matrix[i][j] + std::cmp::min(dp[j], std::cmp::min(dp[j - 1], dp[j + 1]));
                }
            }
            dp = temp;
        }

        return dp.into_iter().min().unwrap();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_falling_path_sum(
            [[2, 1, 3], [6, 5, 4], [7, 8, 9]]
                .map(|a| a.to_vec())
                .to_vec()
        )
    ); // 13;
    println!(
        "{:?}",
        Solution::min_falling_path_sum([[-19, 57], [-40, -5]].map(|a| a.to_vec()).to_vec())
    ); // -59;
}

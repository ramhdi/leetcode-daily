// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/
// 2024/10/29

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for j in (0..n - 1).rev() {
            for i in 0..m {
                if grid[i][j + 1] > grid[i][j] {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i][j + 1] + 1);
                }

                if i > 0 && grid[i - 1][j + 1] > grid[i][j] {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j + 1] + 1);
                }

                if i < m - 1 && grid[i + 1][j + 1] > grid[i][j] {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i + 1][j + 1] + 1);
                }
            }
        }

        dp.iter().map(|row| row[0]).max().unwrap_or(0)
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::max_moves(
            [[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 3

    println!(
        "{:?}",
        Solution::max_moves(
            [[3, 2, 4], [2, 1, 9], [1, 1, 7]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 0
}

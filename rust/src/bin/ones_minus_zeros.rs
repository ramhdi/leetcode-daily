// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
// 2023/12/14

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result: Vec<Vec<i32>> = vec![vec![0; n]; m];
        let mut sum_rows: Vec<i32> = vec![0; m];
        let mut sum_cols: Vec<i32> = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                sum_rows[i] += 2 * grid[i][j] - 1;
                sum_cols[j] += 2 * grid[i][j] - 1;
            }
        }

        for i in 0..m {
            for j in 0..n {
                result[i][j] = sum_rows[i] + sum_cols[j];
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]])
    );
    println!(
        "{:?}",
        Solution::ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]])
    );
    println!(
        "{:?}",
        Solution::ones_minus_zeros(vec![vec![1, 0, 0, 0], vec![0, 0, 1, 0], vec![1, 0, 0, 1]])
    );
}

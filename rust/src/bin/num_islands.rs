// https://leetcode.com/problems/number-of-islands/
// 2024/04/19

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut result: i32 = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    Self::dfs(&mut grid, i, j, m, n);
                    result += 1;
                }
            }
        }

        return result;
    }

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] != '1' {
            return;
        }

        grid[r][c] = '0';

        if r > 0 {
            Self::dfs(grid, r - 1, c, rows, cols);
        }

        if r + 1 < rows {
            Self::dfs(grid, r + 1, c, rows, cols);
        }

        if c > 0 {
            Self::dfs(grid, r, c - 1, rows, cols);
        }

        if c + 1 < cols {
            Self::dfs(grid, r, c + 1, rows, cols);
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::num_islands(
            [
                ['1', '1', '1', '1', '0'],
                ['1', '1', '0', '1', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '0', '0', '0']
            ]
            .map(|r| r.to_vec())
            .to_vec()
        )
    ); // 1

    println!(
        "{:?}",
        Solution::num_islands(
            [
                ['1', '1', '0', '0', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '1', '0', '0'],
                ['0', '0', '0', '1', '1']
            ]
            .map(|r| r.to_vec())
            .to_vec()
        )
    ); // 3
}

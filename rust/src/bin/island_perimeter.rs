// https://leetcode.com/problems/island-perimeter/
// 2024/04/18

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;

        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if i == 0 || grid[i - 1][j] == 0 {
                        result += 1;
                    }

                    if i == m - 1 || grid[i + 1][j] == 0 {
                        result += 1;
                    }

                    if j == 0 || grid[i][j - 1] == 0 {
                        result += 1;
                    }

                    if j == n - 1 || grid[i][j + 1] == 0 {
                        result += 1;
                    }
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::island_perimeter(
            [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
                .map(|r| r.to_vec())
                .to_vec()
        )
    ); // 16

    println!(
        "{:?}",
        Solution::island_perimeter([[1]].map(|r| r.to_vec()).to_vec())
    ); // 4

    println!(
        "{:?}",
        Solution::island_perimeter([[1, 0]].map(|r| r.to_vec()).to_vec())
    ); // 4
}

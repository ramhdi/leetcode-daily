// https://leetcode.com/problems/out-of-boundary-paths/
// 2024/01/26

const MOD: i32 = 1_000_000_007;

// impl Solution {
//     pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
//         if max_move == 0 {
//             return 0;
//         }

//         if Solution::is_outside(m, n, start_row, start_column) {
//             return 0;
//         }

//         let mut result: i32 = 0;

//         if Solution::is_outside(m, n, start_row - 1, start_column) {
//             result = (result + 1) % MOD;
//         } else {
//             result = (result
//                 + Solution::find_paths(m, n, max_move - 1, start_row - 1, start_column))
//                 % MOD;
//         }

//         if Solution::is_outside(m, n, start_row + 1, start_column) {
//             result = (result + 1) % MOD;
//         } else {
//             result = (result
//                 + Solution::find_paths(m, n, max_move - 1, start_row + 1, start_column))
//                 % MOD;
//         }

//         if Solution::is_outside(m, n, start_row, start_column - 1) {
//             result = (result + 1) % MOD;
//         } else {
//             result = (result
//                 + Solution::find_paths(m, n, max_move - 1, start_row, start_column - 1))
//                 % MOD;
//         }

//         if Solution::is_outside(m, n, start_row, start_column + 1) {
//             result = (result + 1) % MOD;
//         } else {
//             result = (result
//                 + Solution::find_paths(m, n, max_move - 1, start_row, start_column + 1))
//                 % MOD;
//         }

//         return result;
//     }

//     pub fn is_outside(m: i32, n: i32, row: i32, col: i32) -> bool {
//         return row < 0 || row >= m || col < 0 || col >= n;
//     }
// }

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp: Vec<Vec<Vec<i32>>> =
            vec![vec![vec![0; n as usize + 1]; m as usize + 1]; max_move as usize + 1];

        for k in 1..=max_move {
            for i in 0..=m {
                for j in 0..=n {
                    if Solution::is_outside(m, n, i - 1, j) {
                        dp[k as usize][i as usize][j as usize] =
                            (dp[k as usize][i as usize][j as usize] + 1) % MOD;
                    } else {
                        dp[k as usize][i as usize][j as usize] = (dp[k as usize][i as usize]
                            [j as usize]
                            + dp[k as usize - 1][i as usize - 1][j as usize])
                            % MOD;
                    }

                    if Solution::is_outside(m, n, i + 1, j) {
                        dp[k as usize][i as usize][j as usize] =
                            (dp[k as usize][i as usize][j as usize] + 1) % MOD;
                    } else {
                        dp[k as usize][i as usize][j as usize] = (dp[k as usize][i as usize]
                            [j as usize]
                            + dp[k as usize - 1][i as usize + 1][j as usize])
                            % MOD;
                    }

                    if Solution::is_outside(m, n, i, j - 1) {
                        dp[k as usize][i as usize][j as usize] =
                            (dp[k as usize][i as usize][j as usize] + 1) % MOD;
                    } else {
                        dp[k as usize][i as usize][j as usize] = (dp[k as usize][i as usize]
                            [j as usize]
                            + dp[k as usize - 1][i as usize][j as usize - 1])
                            % MOD;
                    }

                    if Solution::is_outside(m, n, i, j + 1) {
                        dp[k as usize][i as usize][j as usize] =
                            (dp[k as usize][i as usize][j as usize] + 1) % MOD;
                    } else {
                        dp[k as usize][i as usize][j as usize] = (dp[k as usize][i as usize]
                            [j as usize]
                            + dp[k as usize - 1][i as usize][j as usize + 1])
                            % MOD;
                    }
                }
            }
        }

        return dp[max_move as usize][start_row as usize][start_column as usize];
    }

    pub fn is_outside(m: i32, n: i32, row: i32, col: i32) -> bool {
        return row < 0 || row >= m || col < 0 || col >= n;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_paths(2, 2, 2, 0, 0)); // 6
    println!("{:?}", Solution::find_paths(1, 3, 3, 0, 1)); // 12
    println!("{:?}", Solution::find_paths(8, 7, 16, 1, 5)); // 102984580
    println!("{:?}", Solution::find_paths(50, 50, 50, 25, 25)); // 276775132
}

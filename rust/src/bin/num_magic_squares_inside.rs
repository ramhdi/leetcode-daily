// https://leetcode.com/problems/integer-to-english-words/
// 2024/08/09

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid[0].len(), grid.len());
        if m < 3 || n < 3 {
            return 0;
        }

        let mut result: i32 = 0;

        for i in 1..(n - 1) {
            for j in 1..(m - 1) {
                let mut one_nine = [0; 9];
                'outer: for k in (i - 1)..=(i + 1) {
                    for l in (j - 1)..=(j + 1) {
                        if grid[k][l] == 0 || grid[k][l] > 9 {
                            break 'outer;
                        }

                        one_nine[grid[k][l] as usize - 1] += 1;
                    }
                }

                let is_19 = one_nine.iter().find(|&&n| n != 1) == None;
                let matrix_slice = [
                    &grid[i - 1][(j - 1)..=(j + 1)],
                    &grid[i][(j - 1)..=(j + 1)],
                    &grid[i + 1][(j - 1)..=(j + 1)],
                ];

                if is_19 && Self::check_equal_sums(&matrix_slice) {
                    result += 1;
                }
            }
        }

        result
    }

    fn check_equal_sums(matrix: &[&[i32]; 3]) -> bool {
        let reference_sum = matrix[0].iter().sum::<i32>();

        // Check rows
        for row in 1..3 {
            if matrix[row].iter().sum::<i32>() != reference_sum {
                return false;
            }
        }

        // Check columns
        for col in 0..3 {
            let mut col_sum = 0;
            for row in 0..3 {
                col_sum += matrix[row][col];
            }
            if col_sum != reference_sum {
                return false;
            }
        }

        // Check diagonals
        let mut diag1_sum = 0;
        let mut diag2_sum = 0;
        for i in 0..3 {
            diag1_sum += matrix[i][i];
            diag2_sum += matrix[i][2 - i];
        }
        if diag1_sum != reference_sum || diag2_sum != reference_sum {
            return false;
        }

        true
    }
}

struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::num_magic_squares_inside(
            [[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]
                .map(|v| v.to_vec())
                .to_vec()
        )
    ); // 1

    println!(
        "{}",
        Solution::num_magic_squares_inside([[8]].map(|v| v.to_vec()).to_vec())
    ); // 0

    println!(
        "{}",
        Solution::num_magic_squares_inside(
            [[4, 7, 8], [9, 5, 1], [2, 3, 6]]
                .map(|v| v.to_vec())
                .to_vec()
        )
    ); // 0
}

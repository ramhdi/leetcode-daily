// https://leetcode.com/problems/search-a-2d-matrix/

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i: usize = 0;
        let mut found_row = false;

        while i < matrix.len() - 1 && !found_row {
            if target == matrix[i][0] || target == matrix[i + 1][0] {
                return true;
            } else if target > matrix[i][0] && target < matrix[i + 1][0] {
                found_row = true;
            } else {
                i += 1;
            }
        }

        let mut j: usize = 0;
        while j < matrix[i].len() {
            if matrix[i][j] == target {
                return true;
            } else {
                j += 1;
            }
        }
        return false;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        )
    );
    println!(
        "{}",
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        )
    );
    println!("{}", Solution::search_matrix(vec![vec![1], vec![3]], 3));
}

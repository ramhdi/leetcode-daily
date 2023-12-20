// https://leetcode.com/problems/transpose-matrix/

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                result[j][i] = matrix[i][j];
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!(
        "{:?}",
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]])
    );
}

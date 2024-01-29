// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
// 2024/01/28

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut count = 0;

        for left in 0..cols {
            let mut row_sum = vec![0; rows];
            for right in left..cols {
                for row in 0..rows {
                    row_sum[row] += matrix[row][right];
                }
                count += Self::count_subarrays_with_sum(&row_sum, target);
            }
        }

        return count;
    }

    fn count_subarrays_with_sum(nums: &[i32], target: i32) -> i32 {
        let mut prefix_sum = HashMap::new();
        prefix_sum.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;

        for &num in nums {
            sum += num;
            if let Some(&prev_count) = prefix_sum.get(&(sum - target)) {
                count += prev_count;
            }
            *prefix_sum.entry(sum).or_insert(0) += 1;
        }

        return count;
    }
}
pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::num_submatrix_sum_target(
            [[0, 1, 0], [1, 1, 1], [0, 1, 0]]
                .map(|r| r.to_vec())
                .to_vec(),
            0
        )
    ); // 4

    println!(
        "{:?}",
        Solution::num_submatrix_sum_target([[1, -1], [-1, 1]].map(|r| r.to_vec()).to_vec(), 0)
    ); // 5

    println!(
        "{:?}",
        Solution::num_submatrix_sum_target([[904]].map(|r| r.to_vec()).to_vec(), 0)
    ); // 0
}

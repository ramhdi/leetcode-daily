// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// 2024/11/21

// impl Solution {
//     pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
//         let mut result = 0;

//         for i in 0..matrix.len() {
//             let mut same_or_opposite_rows = 0;
//             for j in 0..matrix.len() {
//                 if i == j {
//                     same_or_opposite_rows += 1;
//                 } else {
//                     same_or_opposite_rows +=
//                         Self::same_or_opposite_row(&matrix[i], &matrix[j]) as i32;
//                 }
//             }

//             result = result.max(same_or_opposite_rows);
//         }

//         result
//     }

//     fn same_or_opposite_row(row1: &[i32], row2: &[i32]) -> bool {
//         row1.iter().zip(row2.iter()).all(|e| e.0 == e.1)
//             || row1.iter().zip(row2.iter()).all(|e| e.0 != e.1)
//     }
// }

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        *matrix
            .iter()
            .map(|row| {
                let first = row[0];
                let pattern: Vec<i32> = row.iter().map(|&x| x ^ first).collect();
                pattern
            })
            .fold(HashMap::<Vec<i32>, i32>::new(), |mut map, pattern| {
                *map.entry(pattern).or_insert(0) += 1;
                map
            })
            .values()
            .max()
            .unwrap_or(&0) as i32
    }
}
struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_equal_rows_after_flips([[0, 1], [1, 1]].map(|e| e.to_vec()).to_vec())
    ); // 1

    println!(
        "{:?}",
        Solution::max_equal_rows_after_flips([[0, 1], [1, 0]].map(|e| e.to_vec()).to_vec())
    ); // 2

    println!(
        "{:?}",
        Solution::max_equal_rows_after_flips(
            [[0, 0, 0], [0, 0, 1], [1, 1, 0]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 2
}

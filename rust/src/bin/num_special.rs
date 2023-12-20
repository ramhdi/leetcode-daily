// https://leetcode.com/problems/special-positions-in-a-binary-matrix
// 2023/12/13

use std::collections::HashMap;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let m = mat.len();
        let n = mat[0].len();
        let mut one_rows: HashMap<usize, i32> = HashMap::new();
        let mut one_cols: HashMap<usize, i32> = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                let counter_row = one_rows.entry(i).or_insert(0);
                let counter_col = one_cols.entry(j).or_insert(0);
                if mat[i][j] == 1 {
                    *counter_row += 1;
                    *counter_col += 1;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if one_rows.get(&i).unwrap() == &1
                    && one_cols.get(&j).unwrap() == &1
                    && mat[i][j] == 1
                {
                    result += 1;
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]])
    );
    println!(
        "{}",
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
    );
    println!(
        "{}",
        Solution::num_special(vec![vec![1, 0, 0, 0], vec![0, 0, 1, 0], vec![1, 0, 0, 1]])
    );
}

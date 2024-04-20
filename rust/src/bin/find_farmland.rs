// https://leetcode.com/problems/find-all-groups-of-farmland/
// 2024/04/20

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let m = land.len();
        let n = land[0].len();

        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 1
                    && (i == 0 || land[i - 1][j] == 0)
                    && (j == 0 || land[i][j - 1] == 0)
                {
                    let mut end_i = i;
                    let mut end_j = j;

                    while end_i + 1 < m && land[end_i + 1][j] == 1 {
                        end_i += 1;
                    }

                    while end_j + 1 < n && land[i][end_j + 1] == 1 {
                        end_j += 1;
                    }

                    result.push(vec![i as i32, j as i32, end_i as i32, end_j as i32]);
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
        Solution::find_farmland(
            [[1, 0, 0], [0, 1, 1], [0, 1, 1]]
                .map(|r| r.to_vec())
                .to_vec()
        )
    ); // [[0,0,0,0],[1,1,2,2]]

    println!(
        "{:?}",
        Solution::find_farmland([[1, 1], [1, 1]].map(|r| r.to_vec()).to_vec())
    ); // [[0,0,1,1]]

    println!(
        "{:?}",
        Solution::find_farmland([[0]].map(|r| r.to_vec()).to_vec())
    ); // []
}

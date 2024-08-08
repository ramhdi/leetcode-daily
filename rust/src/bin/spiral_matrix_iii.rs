// https://leetcode.com/problems/spiral-matrix-iii/
// 2024/08/08

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let grid = (rows * cols) as usize;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(grid);
        result.push(vec![r_start, c_start]);
        let mut curr_r = r_start;
        let mut curr_c = c_start;
        let dirs = ["rightdown", "leftup"];
        let mut step = 1;
        let mut dir: usize = 0;

        while result.len() < grid {
            match dirs[dir] {
                "rightdown" => {
                    for _ in 0..step {
                        curr_c += 1;
                        if curr_r >= 0 && curr_r < rows && curr_c >= 0 && curr_c < cols {
                            result.push(vec![curr_r, curr_c]);
                        }
                    }

                    for _ in 0..step {
                        curr_r += 1;
                        if curr_r >= 0 && curr_r < rows && curr_c >= 0 && curr_c < cols {
                            result.push(vec![curr_r, curr_c]);
                        }
                    }
                }
                "leftup" => {
                    for _ in 0..step {
                        curr_c -= 1;
                        if curr_r >= 0 && curr_r < rows && curr_c >= 0 && curr_c < cols {
                            result.push(vec![curr_r, curr_c]);
                        }
                    }

                    for _ in 0..step {
                        curr_r -= 1;
                        if curr_r >= 0 && curr_r < rows && curr_c >= 0 && curr_c < cols {
                            result.push(vec![curr_r, curr_c]);
                        }
                    }
                }
                _ => (),
            }

            dir = (dir + 1) % 2;
            step += 1;
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::spiral_matrix_iii(1, 4, 0, 0)); // [[0,0],[0,1],[0,2],[0,3]]

    println!("{:?}", Solution::spiral_matrix_iii(5, 6, 1, 4)); // [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]
}

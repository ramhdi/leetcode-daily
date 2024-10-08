// https://leetcode.com/problems/walking-robot-simulation/
// 2024/09/04

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut pos = (0, 0);
        let mut dir = 0;
        let obstacles: HashSet<(i32, i32)> = obstacles.into_iter().map(|o| (o[0], o[1])).collect();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // if obstacles.contains(&(0, 0)) {
        //     return 0;
        // }

        for c in commands {
            match c {
                -2 => dir = (dir + 3) % 4,
                -1 => dir = (dir + 1) % 4,
                _ => {
                    for _ in 0..c {
                        let next_pos = (pos.0 + directions[dir].0, pos.1 + directions[dir].1);

                        if obstacles.contains(&next_pos) {
                            break;
                        }

                        pos = next_pos;
                        result = result.max(pos.0 * pos.0 + pos.1 * pos.1);
                    }
                }
            }
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::robot_sim([4, -1, 3].to_vec(), [].to_vec())); // 25

    println!(
        "{}",
        Solution::robot_sim(
            [4, -1, 4, -2, 4].to_vec(),
            [[2, 4]].map(|e| e.to_vec()).to_vec()
        )
    ); // 65

    println!(
        "{}",
        Solution::robot_sim([6, -1, -1, 6].to_vec(), [].to_vec())
    ); // 36
}

// https://leetcode.com/problems/path-crossing/
// 2023/12/23

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut point: (i32, i32) = (0, 0);
        let mut visited_points: HashSet<(i32, i32)> = HashSet::new();
        visited_points.insert(point);

        for p in path.chars() {
            match p {
                'N' => {
                    point.1 += 1;
                }
                'E' => {
                    point.0 += 1;
                }
                'S' => {
                    point.1 -= 1;
                }
                'W' => {
                    point.0 -= 1;
                }
                _ => (),
            }

            if visited_points.contains(&point) {
                return true;
            }
            visited_points.insert(point);
        }
        return false;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::is_path_crossing("NES".to_string()));
    println!("{}", Solution::is_path_crossing("NESWW".to_string()));
}

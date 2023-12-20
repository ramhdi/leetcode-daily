// https://leetcode.com/problems/find-center-of-star-graph/

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        for j in 0..2 {
            if edges[0][j] == edges[1][j] || edges[0][j] == edges[1][(j + 1) % 2] {
                return edges[0][j];
            }
        }
        return 0;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]])
    );
}

// https://leetcode.com/problems/minimum-time-visiting-all-points/

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 {
            return 0;
        }

        let mut result: i32 = 0;
        for i in 0..points.len() - 1 {
            result += std::cmp::max(
                (points[i][0] - points[i + 1][0]).abs(),
                (points[i][1] - points[i + 1][1]).abs(),
            );
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]])
    );
}

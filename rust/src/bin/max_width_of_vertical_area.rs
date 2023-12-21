// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/

use std::collections::BTreeSet;

impl Solution {
    pub fn _max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 2 {
            return (points[1][0] - points[0][0]).abs();
        }

        let mut result: i32 = 0;
        let points_sorted = {
            let mut temp = points;
            temp.sort_by(|a, b| a[0].cmp(&b[0]));
            temp
        };

        for i in 0..points_sorted.len() - 1 {
            result = result.max(points_sorted[i + 1][0] - points_sorted[i][0]);
        }
        return result;
    }

    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 2 {
            return (points[1][0] - points[0][0]).abs();
        }

        let mut result: i32 = 0;
        let mut points_x: BTreeSet<i32> = BTreeSet::new();
        for point in points {
            points_x.insert(point[0]);
        }

        let mut temp: i32 = points_x.pop_first().unwrap();
        while !points_x.is_empty() {
            let temp2: i32 = points_x.pop_first().unwrap();
            result = result.max(temp2 - temp);
            temp = temp2;
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]])
    );
    println!(
        "{:?}",
        Solution::max_width_of_vertical_area(vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8]
        ])
    );
    println!(
        "{:?}",
        Solution::max_width_of_vertical_area(vec![vec![3, 1], vec![1, 4],])
    );
}

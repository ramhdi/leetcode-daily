// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
// 2024/03/18

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }

        let mut points = points;
        points.sort_by_key(|x| x[1]);

        let mut arrows = 1;
        let mut prev_end = points[0][1];

        for point in points.iter().skip(1) {
            let start = point[0];
            let end = point[1];

            if start > prev_end {
                arrows += 1;
                prev_end = end;
            }
        }

        return arrows;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_min_arrow_shots(
            [[10, 16], [2, 8], [1, 6], [7, 12]]
                .map(|p| p.to_vec())
                .to_vec(),
        )
    ); // 2

    println!(
        "{:?}",
        Solution::find_min_arrow_shots(
            [[1, 2], [3, 4], [5, 6], [7, 8]]
                .map(|p| p.to_vec())
                .to_vec(),
        )
    ); // 4

    println!(
        "{:?}",
        Solution::find_min_arrow_shots(
            [[1, 2], [2, 3], [3, 4], [4, 5]]
                .map(|p| p.to_vec())
                .to_vec(),
        )
    ); // 2
}

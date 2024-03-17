// https://leetcode.com/problems/insert-interval/
// 2024/03/17

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut new_interval = new_interval;

        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }

        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);

        while i < intervals.len() {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::insert(
            [[1, 3], [6, 9]].map(|p| p.to_vec()).to_vec(),
            [2, 5].to_vec()
        )
    ); // [[1,5],[6,9]]

    println!(
        "{:?}",
        Solution::insert(
            [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
                .map(|p| p.to_vec())
                .to_vec(),
            [4, 8].to_vec()
        )
    ); // [[1,2],[3,10],[12,16]]

    println!(
        "{:?}",
        Solution::insert([[1, 5]].map(|p| p.to_vec()).to_vec(), [2, 3].to_vec())
    ); // [[1,5]]
}

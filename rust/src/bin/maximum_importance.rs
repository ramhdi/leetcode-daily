// https://leetcode.com/problems/maximum-total-importance-of-roads/
// 2024/06/28

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degrees: Vec<i64> =
            roads
                .iter()
                .fold(vec![0i64; n as usize], |mut counts, road| {
                    counts[road[0] as usize] += 1;
                    counts[road[1] as usize] += 1;
                    counts
                });

        degrees.sort_unstable();

        let mut total_importance = 0;
        for i in 1..=n {
            total_importance += degrees[i as usize - 1] * i as i64
        }

        total_importance
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::maximum_importance(
            5,
            [[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]
                .map(|p| p.to_vec())
                .to_vec()
        )
    ); // 43

    println!(
        "{:?}",
        Solution::maximum_importance(5, [[0, 3], [2, 4], [1, 3]].map(|p| p.to_vec()).to_vec())
    ); // 20
}

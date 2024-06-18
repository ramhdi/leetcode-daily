// https://leetcode.com/problems/most-profit-assigning-work/description/
// 2024/06/18

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
        jobs.sort_unstable_by_key(|j| j.0);
        let mut workers = worker;
        workers.sort_unstable();

        let mut max_profit: i32 = 0;
        let mut max_profit_so_far: i32 = 0;
        let mut i = 0;

        for w in workers {
            while i < jobs.len() && w >= jobs[i].0 {
                max_profit_so_far = std::cmp::max(max_profit_so_far, jobs[i].1);
                i += 1;
            }
            max_profit += max_profit_so_far;
        }

        return max_profit;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_profit_assignment(
            [2, 4, 6, 8, 10].to_vec(),
            [10, 20, 30, 40, 50].to_vec(),
            [4, 5, 6, 7].to_vec()
        )
    ); // 100

    println!(
        "{:?}",
        Solution::max_profit_assignment(
            [85, 47, 57].to_vec(),
            [24, 66, 99].to_vec(),
            [40, 25, 25].to_vec()
        )
    ); // 0
}

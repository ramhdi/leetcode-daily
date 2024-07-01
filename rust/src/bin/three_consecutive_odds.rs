// https://leetcode.com/problems/three-consecutive-odds/
// 2024/07/01

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3)
            .any(|window| window.iter().all(|&x| x % 2 != 0))
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::three_consecutive_odds([2, 6, 4, 1].to_vec())
    ); // false

    println!(
        "{:?}",
        Solution::three_consecutive_odds([1, 2, 34, 3, 4, 5, 7, 23, 12].to_vec())
    ); // true
}

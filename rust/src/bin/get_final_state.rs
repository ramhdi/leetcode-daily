// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/
// 2024/12/16

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let n = nums.len();
        let mut pq: BinaryHeap<Reverse<(i32, usize)>> = nums
            .into_iter()
            .enumerate()
            .map(|(i, val)| Reverse((val, i)))
            .collect();

        for _ in 0..k {
            let Reverse((min_val, idx)) = pq.pop().unwrap();
            pq.push(Reverse((min_val * multiplier, idx)));
        }

        let mut result = vec![0; n];
        while let Some(Reverse((val, idx))) = pq.pop() {
            result[idx] = val;
        }

        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::get_final_state([2, 1, 3, 5, 6].to_vec(), 5, 2)
    ); // [8,4,6,5,6]

    println!("{:?}", Solution::get_final_state([1, 2].to_vec(), 3, 4)); // [16,8]
}

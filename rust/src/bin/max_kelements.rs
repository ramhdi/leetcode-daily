// https://www.geeksforgeeks.org/priority-queue-using-binary-heap/
// 2024/10/14

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut score: i64 = 0;
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums);

        for _ in 0..k {
            let greatest = heap.pop().unwrap();
            score += greatest as i64;
            heap.push(Self::ceiling_division(greatest, 3));
        }

        score
    }

    fn ceiling_division(dividend: i32, divisor: i32) -> i32 {
        (dividend + divisor - 1) / divisor
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_kelements([10, 10, 10, 10, 10].to_vec(), 5)
    ); // 50

    println!(
        "{:?}",
        Solution::max_kelements([1, 10, 3, 3, 3].to_vec(), 3)
    ); // 17
}

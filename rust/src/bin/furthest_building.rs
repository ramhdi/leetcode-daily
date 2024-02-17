// https://leetcode.com/problems/furthest-building-you-can-reach/
// 2024/02/17

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut diff_heap = BinaryHeap::new();
        for i in 0..heights.len() - 1 {
            let diff = heights[i + 1] - heights[i];
            if diff <= 0 {
                continue;
            }

            bricks -= diff;
            diff_heap.push(diff);

            if bricks < 0 {
                bricks += diff_heap.pop().unwrap();
                ladders -= 1;
            }

            if ladders < 0 {
                return i as i32;
            }
        }

        return heights.len() as i32 - 1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::furthest_building([4, 2, 7, 6, 9, 14, 12].to_vec(), 5, 1)
    ); // 4

    println!(
        "{:?}",
        Solution::furthest_building([4, 12, 2, 7, 3, 18, 20, 3, 19].to_vec(), 10, 2)
    ); // 7

    println!(
        "{:?}",
        Solution::furthest_building([14, 3, 19, 3].to_vec(), 17, 0)
    ); // 3
}

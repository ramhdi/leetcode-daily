// https://leetcode.com/problems/squares-of-a-sorted-array/
// 2024/03/02

// use std::collections::BinaryHeap;

// impl Solution {
//     pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//         let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();

//         for n in nums {
//             max_heap.push(n * n);
//         }
//         return max_heap.into_sorted_vec();
//     }
// }

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = Vec::new();

        while left <= right {
            let l_v = nums[left];
            let l_sq = l_v * l_v;

            let r_v = nums[right];
            let r_sq = r_v * r_v;

            if l_sq >= r_sq {
                result.push(l_sq);
                left += 1;
            } else {
                result.push(r_sq);
                right -= 1;
            }
        }

        result.reverse();
        result
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_squares([-4, -1, 0, 3, 10].to_vec())
    ); // [0,1,9,16,100]

    println!(
        "{:?}",
        Solution::sorted_squares([-7, -3, 2, 3, 11].to_vec())
    ); // [4,9,9,49,121]
}

// https://leetcode.com/problems/intersection-of-two-arrays/
// 2024/03/10

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let nums1: HashSet<i32> = nums1.into_iter().collect();
        let nums2: HashSet<i32> = nums2.into_iter().collect();

        for n in nums1 {
            if nums2.contains(&n) {
                result.push(n);
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::intersection([1, 2, 2, 1].to_vec(), [2, 2].to_vec())
    ); // [2]

    println!(
        "{:?}",
        Solution::intersection([4, 9, 5].to_vec(), [9, 4, 9, 8, 4].to_vec())
    ); // [9,4]
}

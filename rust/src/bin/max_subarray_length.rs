// https://leetcode.com/problems/length-of-result-subarray-with-at-most-k-frequency/
// 2024/03/28

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 1;
        let mut num_count: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        let mut i = 0;
        let mut j = 0;

        while j < nums.len() {
            *num_count.entry(nums[j]).or_insert(0) += 1;

            while *num_count.get(&nums[j]).unwrap() > k {
                *num_count.entry(nums[i]).or_insert(1) -= 1;
                i += 1;
            }

            result = std::cmp::max(result, (j - i + 1) as i32);

            j += 1;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_subarray_length([1, 2, 3, 1, 2, 3, 1, 2].to_vec(), 2)
    ); // 6

    println!(
        "{:?}",
        Solution::max_subarray_length([1, 2, 1, 2, 1, 2, 1, 2].to_vec(), 1)
    ); // 2

    println!(
        "{:?}",
        Solution::max_subarray_length([5, 5, 5, 5, 5, 5, 5].to_vec(), 4)
    ); // 4

    println!("{:?}", Solution::max_subarray_length([1, 11].to_vec(), 2)); // 2
}

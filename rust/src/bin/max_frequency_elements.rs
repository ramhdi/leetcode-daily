// https://leetcode.com/problems/count-elements-with-maximum-frequency/
// 2024/03/08

use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut result: u8 = 0;
        let mut max_count: u8 = 0;
        let mut num_count: HashMap<u8, u8> = HashMap::with_capacity(nums.len());

        for num in nums {
            *num_count.entry(num as u8).or_insert(0) += 1;
        }

        for (_, count) in num_count.clone() {
            max_count = max_count.max(count);
        }

        for (_, count) in num_count {
            if count == max_count {
                result += max_count;
            }
        }

        return result as i32;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_frequency_elements([1, 2, 2, 3, 1, 4].to_vec())
    ); // 4

    println!(
        "{:?}",
        Solution::max_frequency_elements([1, 2, 3, 4, 5].to_vec())
    ); // 5
}

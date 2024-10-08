// https://leetcode.com/problems/make-sum-divisible-by-p/
// 2024/10/03

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let total_sum: i64 = nums.iter().fold(0, |acc, num| acc + *num as i64);
        let target_remainder = (total_sum % p as i64) as i32;

        if target_remainder == 0 {
            return 0;
        }

        let mut ps_remainder: HashMap<i32, i32> = HashMap::new();
        ps_remainder.insert(0, -1);

        let mut prefix_sum = 0;
        let mut min_len = n as i32;

        for (i, &x) in nums.iter().enumerate() {
            prefix_sum = (prefix_sum + x) % p;

            let needed_remainder = (prefix_sum - target_remainder + p) % p;
            if let Some(&j) = ps_remainder.get(&needed_remainder) {
                min_len = min_len.min((i as i32 - j as i32) as i32);
            }

            ps_remainder.insert(prefix_sum, i as i32);
        }

        if min_len == n as i32 {
            -1
        } else {
            min_len
        }
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_subarray([3, 1, 4, 2].to_vec(), 6)); // 1

    println!("{:?}", Solution::min_subarray([6, 3, 5, 2].to_vec(), 9)); // 2

    println!("{:?}", Solution::min_subarray([1, 2, 3].to_vec(), 3)); // 0
}

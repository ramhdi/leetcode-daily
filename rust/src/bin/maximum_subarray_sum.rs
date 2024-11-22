// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
// 2024/11/19

use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = nums.len();
        if n < k {
            return 0;
        }

        let mut freq: HashMap<i32, usize> = HashMap::new();
        let mut curr_sum: i64 = 0;
        for i in 0..k {
            curr_sum += nums[i] as i64;
            *freq.entry(nums[i]).or_insert(0) += 1;
        }

        let mut max_sum: i64 = if freq.len() == k { curr_sum } else { 0 };

        for i in k..n {
            let left = nums[i - k];
            curr_sum -= left as i64;
            if let Some(count) = freq.get_mut(&left) {
                *count -= 1;
                if *count == 0 {
                    freq.remove(&left);
                }
            }

            let right = nums[i];
            curr_sum += right as i64;
            *freq.entry(right).or_insert(0) += 1;

            if freq.len() == k {
                max_sum = max_sum.max(curr_sum);
            }
        }

        max_sum
    }
}
struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::maximum_subarray_sum([1, 5, 4, 2, 9, 9, 9].to_vec(), 3)
    ); // 15

    println!(
        "{:?}",
        Solution::maximum_subarray_sum([4, 4, 4].to_vec(), 3)
    ); // 0
}

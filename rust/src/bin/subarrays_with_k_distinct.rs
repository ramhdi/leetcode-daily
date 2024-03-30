// https://leetcode.com/problems/subarrays-with-k-different-integers/
// 2024/03/30

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        return Self::subarrays_with_at_most_k_distinct(nums.clone(), k)
            - Self::subarrays_with_at_most_k_distinct(nums.clone(), k - 1);
    }

    fn subarrays_with_at_most_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 {
            return 0;
        }

        if k == 1 {
            return nums.len() as i32;
        }

        let k = k as usize;
        let n = nums.len();
        let mut result: i32 = 0;
        let mut num_count: HashMap<i32, usize> = HashMap::with_capacity(k + 1);
        let mut i = 0;
        let mut j = 0;

        while j < n {
            *num_count.entry(nums[j]).or_insert(0) += 1;

            while num_count.len() > k {
                let count_left = num_count.get_mut(&nums[i]).unwrap();

                *count_left -= 1;

                if *count_left == 0 {
                    num_count.remove(&nums[i]);
                }

                i += 1;
            }

            result += (j - i + 1) as i32;
            j += 1;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::subarrays_with_k_distinct([1, 2, 1, 2, 3].to_vec(), 2)
    ); // 7

    println!(
        "{:?}",
        Solution::subarrays_with_k_distinct([1, 2, 1, 3, 4].to_vec(), 3)
    ); // 3
}

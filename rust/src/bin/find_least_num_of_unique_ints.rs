// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// 2024/02/16

use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k as u32;

        let mut num_count: HashMap<i32, u32> = HashMap::with_capacity(arr.len());

        for num in arr {
            *num_count.entry(num).or_insert(0) += 1;
        }

        let mut count: Vec<u32> = Vec::from_iter(num_count.into_values());
        count.sort_unstable();

        for (i, &c) in count.iter().enumerate() {
            if k >= c {
                k -= c;
            } else {
                return count[i..].len() as i32;
            }
        }

        return 0;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_least_num_of_unique_ints([5, 5, 4].to_vec(), 1)
    ); // 1

    println!(
        "{:?}",
        Solution::find_least_num_of_unique_ints([4, 3, 1, 1, 3, 3, 2].to_vec(), 3)
    ); // 2
}

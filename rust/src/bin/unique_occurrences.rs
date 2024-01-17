// https://leetcode.com/problems/unique-number-of-occurrences/
// 2024/01/17

use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(nums: Vec<i32>) -> bool {
        let mut num_count: HashMap<i32, u16> = HashMap::new();

        for num in nums {
            *num_count.entry(num).or_insert(0) += 1;
        }

        let mut counts: Vec<u16> = Vec::new();
        for (_, v) in num_count {
            if counts.contains(&v) {
                return false;
            }
            counts.push(v);
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::unique_occurrences([1, 2, 2, 1, 1, 3].to_vec())
    ); // true
    println!("{:?}", Solution::unique_occurrences([1, 2].to_vec())); // false
    println!(
        "{:?}",
        Solution::unique_occurrences([-3, 0, 1, -3, 1, 1, 1, -3, 10, 0].to_vec())
    ); // true
}

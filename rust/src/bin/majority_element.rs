// https://leetcode.com/problems/majority-element/

use std::collections::HashMap;

impl Solution {
    pub fn _majority_element(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut candidate: i32 = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }
            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        return candidate;
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut count_max: usize = 0;
        let mut count: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }
        for (n, c) in count.into_iter() {
            if c > count_max {
                result = n;
                count_max = c;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::majority_element(vec![3, 2, 3]));
    println!(
        "{:?}",
        Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2])
    );
}

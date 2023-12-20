// https://leetcode.com/problems/find-all-lonely-numbers-in-the-array/

use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }

        let mut result: Vec<i32> = Vec::new();
        let mut num_count: HashMap<i32, usize> = HashMap::new();

        for num in nums.clone() {
            let entry = num_count.entry(num).or_insert(0);
            *entry += 1;
        }

        for num in nums.clone() {
            if let Some(&count) = num_count.get(&num) {
                if count == 1
                    && num_count.get(&(num as i32 - 1)).is_none()
                    && num_count.get(&(num as i32 + 1)).is_none()
                {
                    result.push(num);
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_lonely(vec![10, 6, 5, 8]));
    println!("{:?}", Solution::find_lonely(vec![1, 3, 5, 3]));
    println!("{:?}", Solution::find_lonely(vec![1]));
}

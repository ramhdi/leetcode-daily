// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums2: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;

            if let Some(&j) = nums2.get(&diff) {
                return vec![j as i32, i as i32];
            }

            nums2.insert(num, i);
        }

        return vec![];
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

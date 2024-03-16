// https://leetcode.com/problems/contiguous-array/
// 2024/03/16

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map_sum: HashMap<i32, i32> = HashMap::new();
        let mut prefix_sum: i32 = 0;
        let mut result: i32 = 0;

        map_sum.insert(0, -1);

        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                prefix_sum -= 1;
            } else {
                prefix_sum += 1;
            }

            if let Some(&j) = map_sum.get(&prefix_sum) {
                result = result.max(i as i32 - j);
            } else {
                map_sum.insert(prefix_sum, i as i32);
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_max_length([0, 1].to_vec())); // 2

    println!("{:?}", Solution::find_max_length([0, 1, 0].to_vec())); // 2
}

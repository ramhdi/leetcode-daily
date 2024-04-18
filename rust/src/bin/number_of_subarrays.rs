// https://leetcode.com/problems/find-the-number-of-subarrays-where-boundary-elements-are-maximum/

use std::collections::VecDeque;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut stack: VecDeque<(i32, i64)> = VecDeque::new();
        let mut res: i64 = 0;

        for a in nums {
            while let Some(&(val, _)) = stack.back() {
                if val < a {
                    stack.pop_back();
                } else {
                    break;
                }
            }
            if stack.is_empty() || stack.back().unwrap().0 != a {
                stack.push_back((a, 0));
            }
            let last_count = &mut stack.back_mut().unwrap().1;
            *last_count += 1;
            res += *last_count;
        }

        res
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::number_of_subarrays([1, 4, 3, 3, 2].to_vec())
    ); // 6

    println!("{:?}", Solution::number_of_subarrays([3, 3, 3].to_vec())); // 6

    println!("{:?}", Solution::number_of_subarrays([6, 26, 6].to_vec())); // 3
}

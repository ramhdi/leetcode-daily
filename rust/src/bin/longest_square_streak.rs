// https://leetcode.com/problems/longest-square-streak-in-an-array/
// 2024/10/28

use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut longest_streak = -1;
        let nums_set: HashSet<i32> = nums.iter().cloned().collect();

        for num in nums {
            let mut square_num = num;
            let mut curr_streak = 1;

            while let Some(next_square) = square_num.checked_mul(square_num) {
                if nums_set.contains(&next_square) {
                    curr_streak += 1;
                    square_num = next_square;
                } else {
                    break;
                }
            }

            longest_streak = longest_streak.max(curr_streak);
        }

        if longest_streak < 2 {
            -1
        } else {
            longest_streak
        }
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::longest_square_streak([4, 3, 6, 16, 8, 2].to_vec())
    ); // 3

    println!(
        "{:?}",
        Solution::longest_square_streak([2, 3, 5, 6, 7].to_vec())
    ); // -1
}

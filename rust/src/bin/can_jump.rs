// https://leetcode.com/problems/jump-game/

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_step: i32 = 0;
        for num in nums {
            if max_step < 0 {
                return false;
            }

            if num > max_step {
                max_step = num;
            }

            max_step -= 1;
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::can_jump([2, 3, 1, 1, 4].to_vec())); // true

    println!("{:?}", Solution::can_jump([3, 2, 1, 0, 4].to_vec())); // false

    println!("{:?}", Solution::can_jump([0, 2, 3].to_vec())); // false
}

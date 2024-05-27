// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/
// 2024/05/27

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = n as i32;

        while left <= right {
            let mid: i32 = left + (right - left) / 2;
            let count: i32 = nums.iter().filter(|&&num| num >= mid).count() as i32;

            if count == mid {
                return mid;
            } else if count > mid {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::special_array([3, 5].to_vec())); // 2

    println!("{:?}", Solution::special_array([0, 0].to_vec())); // -1

    println!("{:?}", Solution::special_array([0, 4, 3, 0, 4].to_vec())); // 3
}

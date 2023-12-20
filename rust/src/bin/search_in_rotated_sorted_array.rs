// https://leetcode.com/problems/search-in-rotated-sorted-array/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return (nums[0] == target) as i32 - 1;
        }
        let result: i32 = -1;

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        while left < right {
            if nums[left] == target {
                return left as i32;
            }
            if nums[right] == target {
                return right as i32;
            }

            let mid: usize = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] < nums[mid] {
                if nums[left] < target && target < nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] < target && target < nums[right] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    println!("{}", Solution::search(vec![1], 2));
    println!("{}", Solution::search(vec![1, 3], 2));
    println!("{}", Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8));
}

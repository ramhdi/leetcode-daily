// https://leetcode.com/problems/container-with-most-water
// LeetCode 75

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut i: usize = 0;
        let mut j: usize = height.len() - 1;
        while i < j {
            let area = std::cmp::min(height[i], height[j]) * (j - i) as i32;
            if area > result {
                result = area;
            }
            if height[i] <= height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

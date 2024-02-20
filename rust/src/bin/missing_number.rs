// https://leetcode.com/problems/missing-number/
// 2024/02/20

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum_n: i32 = n * (n + 1) / 2;
        let sum_num: i32 = nums.iter().sum();
        return sum_n - sum_num;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::missing_number([9, 6, 4, 2, 3, 5, 7, 0, 1].to_vec())
    ); // 8
}

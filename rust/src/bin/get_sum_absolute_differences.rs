// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut left_sum: i32 = 0;
        let mut left_cumsum: Vec<i32> = Vec::new();
        let mut right_sum: i32 = 0;
        let mut right_cumsum: Vec<i32> = Vec::new();

        let n = nums.len();
        for i in 0..n {
            left_sum += nums[i];
            left_cumsum.push(left_sum);

            right_sum += nums[n - i - 1];
            right_cumsum.insert(0, right_sum);
        }

        for i in 0..n {
            result.push(
                (nums[i] * i as i32 - left_cumsum[i])
                    + (right_cumsum[i] - nums[i] * (n - i - 1) as i32),
            )
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10])
    );
}

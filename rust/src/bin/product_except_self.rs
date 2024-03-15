// https://leetcode.com/problems/product-of-array-except-self/
// 2024/03/15

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        let mut left_product = 1;
        for i in 0..n {
            result[i] *= left_product;
            left_product *= nums[i];
        }

        let mut right_product = 1;
        for i in (0..n).rev() {
            result[i] *= right_product;
            right_product *= nums[i];
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::product_except_self([1, 2, 3, 4].to_vec())); // [24,12,8,6]

    println!(
        "{:?}",
        Solution::product_except_self([-1, 1, 0, -3, 3].to_vec())
    ); // [0,0,9,0,0]
}

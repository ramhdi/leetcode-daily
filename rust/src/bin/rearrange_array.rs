// https://leetcode.com/problems/rearrange-array-elements-by-sign/
// 2024/02/14

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.clone();
        let n = nums.len();
        let (mut i, mut j, mut k) = (0 as usize, 0 as usize, 0 as usize);

        while k < n {
            while i < n && nums[i] < 0 {
                i += 1;
            }

            if i < n {
                result[k] = nums[i];
                i += 1;
                k += 1;
            }

            while j < n && nums[j] > 0 {
                j += 1;
            }

            if j < n {
                result[k] = nums[j];
                j += 1;
                k += 1;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::rearrange_array([3, 1, -2, -5, 2, -4].to_vec())
    ); // [3,-2,1,-5,2,-4]

    println!("{:?}", Solution::rearrange_array([-1, 1].to_vec())); // [1,-1]

    println!(
        "{:?}",
        Solution::rearrange_array(
            [28, -41, 22, -8, -37, 46, 35, -9, 18, -6, 19, -26, -37, -10, -9, 15, 14, 31].to_vec()
        )
    ); // [1,-1]
}

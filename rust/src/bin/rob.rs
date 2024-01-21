// https://leetcode.com/problems/house-robber/
// 2024/01/21

impl Solution {
    pub fn _rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }
        if n == 3 {
            return std::cmp::max(nums[0] + nums[2], nums[1]);
        }
        return std::cmp::max(
            nums[0] + Solution::_rob(nums[2..].to_vec()),
            nums[1] + Solution::_rob(nums[3..].to_vec()),
        );
    }
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![0; n];

        for i in 0..n {
            if i == 0 {
                dp[n - 1 - i] = nums[n - 1 - i];
            } else if i == 1 {
                dp[n - 1 - i] = std::cmp::max(nums[n - 1], nums[n - 2]);
            } else if i == 2 {
                dp[n - 1 - i] = std::cmp::max(nums[n - 1] + nums[n - 3], nums[n - 2]);
            } else {
                dp[n - 1 - i] =
                    std::cmp::max(nums[n - 1 - i] + dp[n + 1 - i], nums[n - i] + dp[n + 2 - i]);
            }
        }

        return dp[0];
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::rob([1, 2, 3, 1].to_vec())); // 4
    println!("{:?}", Solution::rob([2, 7, 9, 3, 1].to_vec())); // 12
    println!("{:?}", Solution::rob([2, 1, 1, 2].to_vec())); // 4
    println!(
        "{:?}",
        Solution::rob(
            [
                226, 174, 214, 16, 218, 48, 153, 131, 128, 17, 157, 142, 88, 43, 37, 157, 43, 221,
                191, 68, 206, 23, 225, 82, 54, 118, 111, 46, 80, 49, 245, 63, 25, 194, 72, 80, 143,
                55, 209, 18, 55, 122, 65, 66, 177, 101, 63, 201, 172, 130, 103, 225, 142, 46, 86,
                185, 62, 138, 212, 192, 125, 77, 223, 188, 99, 228, 90, 25, 193, 211, 84, 239, 119,
                234, 85, 83, 123, 120, 131, 203, 219, 10, 82, 35, 120, 180, 249, 106, 37, 169, 225,
                54, 103, 55, 166, 124
            ]
            .to_vec()
        )
    ); // 7102
}

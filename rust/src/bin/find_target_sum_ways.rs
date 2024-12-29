// https://leetcode.com/problems/target-sum/
// 2024/12/26

// Attempt 1: backtracking with brute force: 315ms, 2.24MB
// impl Solution {
//     pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
//         Self::helper(&nums, target, 0, 0)
//     }

//     fn helper(nums: &Vec<i32>, target: i32, index: usize, current_sum: i32) -> i32 {
//         if index == nums.len() {
//             return if current_sum == target {1} else {0};
//         }

//         Self::helper(nums, target, index + 1, current_sum + nums[index]) + Self::helper(nums, target, index + 1, current_sum - nums[index])
//     }
// }

// Attempt 2: backtracking with DP: 0ms, 2.27MB
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum_nums: i32 = nums.iter().sum();
    
        if target.abs() > sum_nums {
            return 0;
        }
        
        if (target + sum_nums) % 2 != 0 {
            return 0;
        }
        
        let subset_sum = (target + sum_nums) / 2;
        let subset_sum = subset_sum as usize;
        
        let mut dp = vec![0; subset_sum + 1];
        dp[0] = 1;
        
        for num in nums {
            for j in (num..=subset_sum as i32).rev() {
                dp[j as usize] += dp[(j - num) as usize];
            }
        }
        
        dp[subset_sum]
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_target_sum_ways([1, 1, 1, 1, 1].to_vec(), 3)
    ); // 5

    println!("{:?}", Solution::find_target_sum_ways([1].to_vec(), 1)); // 1

    println!("{:?}", Solution::find_target_sum_ways([100].to_vec(), -200)); // 0
}

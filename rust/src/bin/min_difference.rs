// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
// 2024/07/03

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }

        nums.sort_unstable();

        (nums[n - 1] - nums[3])
            .min(nums[n - 2] - nums[2])
            .min(nums[n - 3] - nums[1])
            .min(nums[n - 4] - nums[0])
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_difference([5, 3, 2, 4].to_vec())); // 0

    println!("{:?}", Solution::min_difference([1, 5, 0, 10, 14].to_vec())); // 1

    println!("{:?}", Solution::min_difference([3, 100, 20].to_vec())); // 0
}

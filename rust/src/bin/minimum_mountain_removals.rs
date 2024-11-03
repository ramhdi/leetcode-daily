// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/
// 2024/10/30

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        // lis
        let mut lis = vec![1i32; n];
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lis[i] = std::cmp::max(lis[i], lis[j] + 1);
                }
            }
        }

        // lds
        let mut lds = vec![1i32; n];
        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    lds[i] = std::cmp::max(lds[i], lds[j] + 1);
                }
            }
        }

        // longest mountain
        let mut longest_mountain = 0;
        for i in 1..n - 1 {
            if lis[i] > 1 && lds[i] > 1 {
                longest_mountain = longest_mountain.max(lis[i] + lds[i] - 1);
            }
        }

        n as i32 - longest_mountain
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::minimum_mountain_removals([1, 3, 1].to_vec())
    ); // 0

    println!(
        "{:?}",
        Solution::minimum_mountain_removals([2, 1, 1, 5, 6, 2, 3, 1].to_vec())
    ); // 3
}

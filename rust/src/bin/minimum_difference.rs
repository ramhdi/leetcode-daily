// https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let mut even: Vec<i32> = Vec::new();
        let mut odd: Vec<i32> = Vec::new();
        let nums_sorted: Vec<i32> = {
            let mut temp = nums.clone();
            temp.sort();
            temp
        };

        for (i, &num) in nums_sorted.iter().enumerate() {
            if i % 2 == 0 {
                even.push(num);
            } else {
                odd.push(num);
            }
        }

        let sum_even: i32 = even.iter().sum();
        let sum_odd: i32 = odd.iter().sum();

        return (sum_even - sum_odd).abs();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::minimum_difference(vec![2, -1, 0, 4, -2, -9])
    );
}

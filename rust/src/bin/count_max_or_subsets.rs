// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/
// 2024/10/18

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_bitwise_or = nums.iter().fold(0, |acc, &num| acc | num);

        (0..(1 << nums.len()))
            .filter(|i| {
                nums.iter().enumerate().fold(0, |subset_or, (j, &num)| {
                    if i & (1 << j) != 0 {
                        subset_or | num
                    } else {
                        subset_or
                    }
                }) == max_bitwise_or
            })
            .count() as i32
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::count_max_or_subsets([3, 1].to_vec())); // 2

    println!("{:?}", Solution::count_max_or_subsets([2, 2, 2].to_vec())); // 7

    println!(
        "{:?}",
        Solution::count_max_or_subsets([3, 2, 1, 5].to_vec())
    ); // 6
}

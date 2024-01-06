// https://leetcode.com/problems/longest-increasing-subsequence/
// 2024/01/05

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        let mut seq = Vec::with_capacity(nums.len());
        seq.push(nums[0]);

        for &number in nums.iter().skip(1) {
            match number.cmp(seq.last().unwrap()) {
                Ordering::Equal => continue,
                Ordering::Greater => seq.push(number),
                Ordering::Less => match seq.binary_search(&number) {
                    Ok(_) => {}
                    Err(idx) => seq[idx] = number,
                },
            }
        }
        println!("{:?}", seq);
        return seq.len() as i32;
    }

    pub fn _length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![1; n];
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        return *dp.iter().max().unwrap();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::length_of_lis([10, 9, 2, 5, 3, 7, 101, 18].to_vec())
    );
    println!("{:?}", Solution::length_of_lis([0, 1, 0, 3, 2, 3].to_vec()));
    println!(
        "{:?}",
        Solution::length_of_lis([7, 7, 7, 7, 7, 7, 7].to_vec())
    );
}

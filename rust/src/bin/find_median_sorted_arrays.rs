// https://leetcode.com/problems/median-of-two-sorted-arrays/

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged: Vec<i32> = {
            let mut temp = [nums1, nums2].concat();
            temp.sort();
            temp
        };
        let n = merged.len();
        if n % 2 == 1 {
            return merged[n / 2] as f64;
        }
        return (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.00;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3])
    );
}

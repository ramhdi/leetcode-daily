// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
// 2024/05/20

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let n = nums.len();

        for i in 0..(1 << n) {
            let mut subset: Vec<i32> = Vec::new();
            for j in 0..n {
                if i & (1 << j) != 0 {
                    subset.push(nums[j]);
                }
            }

            result += Self::xor_sum(subset);
        }

        return result;
    }

    fn xor_sum(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        nums.iter().fold(0, |acc, &e| acc ^ e)
    }
}

// Editorial solution, most optimized
// impl Solution {
//     pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
//         nums.iter().fold(0, |acc, &e| acc | e) << (nums.len() as i32 - 1)
//     }
// }

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::subset_xor_sum([1, 3].to_vec())); // 6

    println!("{:?}", Solution::subset_xor_sum([5, 1, 6].to_vec())); // 28

    println!(
        "{:?}",
        Solution::subset_xor_sum([3, 4, 5, 6, 7, 8].to_vec())
    ); //480
}

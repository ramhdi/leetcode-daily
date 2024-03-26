// https://leetcode.com/problems/first-missing-positive/
// 2024/03/26

// use std::collections::HashMap;

// impl Solution {
//     pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
//         let mut num_count: HashMap<i32, usize> = HashMap::new();
//         for num in nums {
//             if num > 0 {
//                 *num_count.entry(num).or_insert(0) += 1;
//             }
//         }

//         for n in 1..=i32::MAX {
//             match num_count.get(&n) {
//                 None => return n,
//                 Some(_) => (),
//             }
//         }

//         return -1;
//     }
// }

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let marker = n + 1;

        // Step 1: Normalize the array
        for num in nums.iter_mut() {
            if *num <= 0 || *num > n {
                *num = marker;
            }
        }

        // Step 2: Mark existing numbers
        for i in 0..n as usize {
            let num = nums[i].abs();
            if num <= n {
                nums[num as usize - 1] = -nums[num as usize - 1].abs();
            }
        }

        // Find the first positive number's index + 1, which is the answer
        nums.iter()
            .enumerate()
            .find(|&(_, &num)| num > 0)
            .map_or(n + 1, |(i, _)| i as i32 + 1)
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::first_missing_positive([1, 2, 0].to_vec())); // 3

    println!(
        "{:?}",
        Solution::first_missing_positive([3, 4, -1, 1].to_vec())
    ); // 2

    println!(
        "{:?}",
        Solution::first_missing_positive([7, 8, 9, 11, 12].to_vec())
    ); // 1
}

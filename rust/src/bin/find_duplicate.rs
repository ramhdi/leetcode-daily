// https://leetcode.com/problems/find-the-duplicate-number/
// 2024/03/24

// use std::collections::HashMap;

// impl Solution {
//     pub fn find_duplicate(nums: Vec<i32>) -> i32 {
//         let mut num_count: HashMap<i32, usize> = HashMap::new();
//         for num in nums {
//             *num_count.entry(num).or_insert(0) += 1;
//         }

//         for (n, c) in num_count {
//             if c > 1 {
//                 return n;
//             }
//         }

//         return -1;
//     }
// }

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];

        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];

            if slow == fast {
                break;
            }
        }

        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_duplicate([1, 3, 4, 2, 2].to_vec())); // 2
    println!("{:?}", Solution::find_duplicate([3, 1, 3, 4, 2].to_vec())); // 3
    println!("{:?}", Solution::find_duplicate([3, 3, 3, 3, 3].to_vec())); // 3
}

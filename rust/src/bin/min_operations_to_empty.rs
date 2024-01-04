// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/
// 2024/01/04

use std::collections::HashMap;

impl Solution {
    pub fn is_combination_two_three(n: i32) -> i32 {
        if n < 0 {
            return -1;
        }
        if n % 3 == 0 {
            return n / 3;
        }
        let mut b = 1;
        while b <= n / 2 {
            let remaining = n - 2 * b;
            if remaining % 3 == 0 {
                return b + remaining / 3;
            }
            b += 1;
        }
        if n % 2 == 0 {
            return n / 2;
        }

        return -1;
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return -1;
        }
        let mut result: i32 = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }
        for (_, v) in count {
            if v == 1 {
                return -1;
            }
            // let operation = Solution::is_combination_two_three(v);
            // if operation == -1 {
            //     return -1;
            // }
            // result += operation;
            if v % 3 == 0 {
                result += v / 3;
            } else {
                result += 1 + v / 3;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_operations([2, 3, 3, 2, 2, 4, 2, 3, 4].to_vec())
    );
    println!(
        "{:?}",
        Solution::min_operations([2, 1, 2, 2, 3, 3].to_vec())
    );
    println!(
        "{:?}",
        Solution::min_operations(
            [14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12].to_vec()
        )
    );
    println!(
        "{:?}",
        Solution::min_operations([19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19].to_vec())
    );
}

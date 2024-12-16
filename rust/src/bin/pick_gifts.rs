// https://leetcode.com/problems/take-gifts-from-the-richest-pile/
// 2024/12/12

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = BinaryHeap::from(gifts);
        for _ in 0..k {
            let mut curr_gift = gifts.pop().unwrap();
            curr_gift = Self::sqrt(curr_gift).unwrap();
            gifts.push(curr_gift);
        }

        gifts.iter().fold(0i64, |acc, &g| acc + g as i64)
    }

    /// Newton-Raphson method
    fn sqrt(n: i32) -> Result<i32, String> {
        if n < 0 {
            return Err("Cannot calculate square root of negative number".to_string());
        }

        if n == 0 || n == 1 {
            return Ok(n);
        }

        let mut x = n;
        let mut y = (x + 1) / 2;

        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }

        Ok(x)
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::pick_gifts([25, 64, 9, 4, 100].to_vec(), 4)
    ); // 29

    println!("{:?}", Solution::pick_gifts([1, 1, 1, 1].to_vec(), 4)); // 4
}

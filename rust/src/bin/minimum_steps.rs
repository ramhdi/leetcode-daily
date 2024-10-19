// https://leetcode.com/problems/separate-black-and-white-balls/
// 2024/10/15

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        s.chars()
            .rev()
            .fold((0i64, 0i64), |(mut steps, mut zeros), c| {
                match c {
                    '0' => zeros += 1,
                    '1' => steps += zeros,
                    _ => (),
                }
                (steps, zeros)
            })
            .0
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::minimum_steps("101".to_string())); // 1

    println!("{:?}", Solution::minimum_steps("100".to_string())); // 2

    println!("{:?}", Solution::minimum_steps("0111".to_string())); // 0
}

// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// 2023/12/24

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }

        let mut start_zero: char = '0';
        let mut change_zero: i32 = 0;
        let mut start_one: char = '1';
        let mut change_one: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            match i % 2 {
                0 => {
                    start_zero = '0';
                    start_one = '1';
                }
                1 => {
                    start_zero = '1';
                    start_one = '0';
                }
                _ => (),
            }

            if c != start_zero {
                change_zero += 1;
            }
            if c != start_one {
                change_one += 1;
            }
        }

        return std::cmp::min(change_zero, change_one);
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::min_operations("0100".to_string()));
    println!("{}", Solution::min_operations("10".to_string()));
    println!("{}", Solution::min_operations("1111".to_string()));
}

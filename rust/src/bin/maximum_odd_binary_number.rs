// Maximum Odd Binary Number
// 2024/03/01

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut result: String = String::new();

        let mut count_zero: usize = 0;
        let mut count_one: usize = 0;

        for c in s.chars() {
            match c {
                '0' => count_zero += 1,
                '1' => count_one += 1,
                _ => (),
            }
        }

        result.push('1');
        count_one -= 1;

        for _ in 0..count_zero {
            result.push('0');
        }

        for _ in 0..count_one {
            result.push('1');
        }

        return result.chars().rev().collect();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::maximum_odd_binary_number("010".to_string())
    ); // 001

    println!(
        "{:?}",
        Solution::maximum_odd_binary_number("0101".to_string())
    ); // 1001
}

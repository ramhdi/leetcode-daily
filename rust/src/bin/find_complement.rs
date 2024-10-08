// https://leetcode.com/problems/number-complement/
// 2024/08/22

impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut result: i32 = 0;
        let mut base: i32 = 1;

        while num > 0 {
            let parity = num % 2;
            result += base * (-parity + 1);
            base *= 2;
            num >>= 1;
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::find_complement(5)); // 2

    println!("{}", Solution::find_complement(1)); // 0
}

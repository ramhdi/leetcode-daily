// https://leetcode.com/problems/number-of-digit-one/

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut result: i32 = 0;

        for i in 1..=n {
            let mut n2 = i;
            while n2 > 0 {
                if n2 % 10 == 1 {
                    result += 1;
                }
                n2 = n2 / 10;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::count_digit_one(13));
}

// https://leetcode.com/problems/power-of-two/

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 1 || n == 3 {
            return true;
        }
        if n < 1 || n % 3 != 0 {
            return false;
        }
        return Solution::is_power_of_three(n / 3);
    }

    pub fn is_power_of_two(n: i32) -> bool {
        return n > 0 && 1162261467 % n == 0;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::is_power_of_three(1));
    println!("{}", Solution::is_power_of_three(27));
    println!("{}", Solution::is_power_of_three(28));
}

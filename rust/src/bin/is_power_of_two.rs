// https://leetcode.com/problems/power-of-two/

impl Solution {
    pub fn _is_power_of_two(n: i32) -> bool {
        if n == 1 || n == 2 {
            return true;
        }
        if n < 1 || n % 2 == 1 {
            return false;
        }
        return Solution::is_power_of_two(n / 2);
    }

    pub fn is_power_of_two(n: i32) -> bool {
        return n > 0 && n & n - 1 == 0;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::is_power_of_two(1));
    println!("{}", Solution::is_power_of_two(16));
    println!("{}", Solution::is_power_of_two(15));
}

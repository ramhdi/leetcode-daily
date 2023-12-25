// https://leetcode.com/problems/powx-n/

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        if n > 0 {
            return x * Solution::my_pow(x, n - 1);
        }
        if n < 0 {
            return 1.0 / Solution::my_pow(x, -n);
        }
        return 1.0;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::my_pow(2.00000, 10));
    println!("{}", Solution::my_pow(2.00000, 3));
    println!("{}", Solution::my_pow(2.00000, -2));
}

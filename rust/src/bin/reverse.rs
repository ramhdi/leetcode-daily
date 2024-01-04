// https://leetcode.com/problems/reverse-integer/

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result: i32 = 0;
        while x.abs() > 0 {
            result += x % 10;
            x /= 10;
            if x.abs() > 0 {
                if result.abs() > i32::MAX / 10 {
                    return 0;
                }
                result *= 10;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::reverse(123));
    println!("{:?}", Solution::reverse(-123));
    println!("{:?}", Solution::reverse(120));
    println!("{:?}", Solution::reverse(1534236469));
}

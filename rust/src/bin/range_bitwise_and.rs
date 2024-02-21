// https://leetcode.com/problems/bitwise-and-of-numbers-range/
// 2024/02/21

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == 0 {
            return 0;
        }

        if left == right {
            return left;
        }

        if left >= 1073741824 && right == 2147483647 {
            return left;
        }

        let mut result: i32 = left;
        for n in (left + 1)..=right {
            result = result & n;
            if result == 0 {
                return 0;
            }
        }

        return result;
    }

    pub fn _range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }

        let mut bit = 1 << 30;
        let mut result = 0;
        for _ in 0..31 {
            if bit & left != bit & right {
                return result;
            }

            if bit & left == bit {
                result |= bit;
            }
            bit >>= 1;
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::range_bitwise_and(5, 7)); // 4
    println!("{:?}", Solution::range_bitwise_and(0, 0)); // 0
    println!("{:?}", Solution::range_bitwise_and(1, 2147483647)); // 0
}

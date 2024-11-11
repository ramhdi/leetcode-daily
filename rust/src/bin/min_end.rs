// https://leetcode.com/problems/minimum-array-end/
// 2024/11/09

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut result = x as i64;
        let mut mask = 1;
        let mut v = n - 1;

        while v > 0 {
            if result & mask == 0 {
                if v & 1 != 0 {
                    result |= mask;
                }

                v >>= 1;
            }

            mask <<= 1;
        }

        result
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::min_end(3, 4)); // 6

    println!("{:?}", Solution::min_end(2, 7)); // 15
}

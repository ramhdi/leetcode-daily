// https://leetcode.com/problems/find-the-pivot-integer/
// 2024/03/13

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let sum = n * (n + 1) / 2;
        let mut low = 1;
        let mut high = n;
        while low <= high {
            let mid = low + (high - low) / 2;
            let mid_square = mid * mid;

            if mid_square == sum {
                return mid;
            } else if mid_square < sum {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        return -1;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::pivot_integer(8)); // 6
    println!("{:?}", Solution::pivot_integer(1)); // 1
    println!("{:?}", Solution::pivot_integer(4)); // -1
}

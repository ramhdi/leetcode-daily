// https://leetcode.com/problems/maximum-swap/
// 2024/10/17

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let n = Self::find_digits(num);
        let mut max_num = num;

        for i in 0..n - 1 {
            for j in i + 1..n {
                max_num = max_num.max(Self::swap(num, i, j));
            }
        }

        max_num
    }

    fn swap(mut num: i32, p1: usize, p2: usize) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        while num > 0 {
            nums.push(num % 10);
            num /= 10;
        }

        let n = nums.len();
        nums.swap(n - p1 - 1, n - p2 - 1);
        nums.iter().rev().fold(0, |acc, n| 10 * acc + n)
    }

    fn find_digits(mut num: i32) -> usize {
        let mut digits = 0;
        while num > 0 {
            digits += 1;
            num /= 10;
        }

        digits
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::maximum_swap(2736)); // 7236

    println!("{:?}", Solution::maximum_swap(9973)); // 9973
}

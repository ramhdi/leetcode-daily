// https://leetcode.com/problems/sequential-digits/
// 2024/02/02

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for start in 1..=8 {
            let digits = 10 - start;
            let mut num: i32 = start;
            for next in (start + 1)..(start + digits) {
                num = 10 * num + next;
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }

        result.sort_unstable();
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::sequential_digits(100, 300)); // [123,234]
    println!("{:?}", Solution::sequential_digits(1000, 13000)); // [1234,2345,3456,4567,5678,6789,12345]
}

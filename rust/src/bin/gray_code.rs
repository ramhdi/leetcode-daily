// https://leetcode.com/problems/gray-code/

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = vec![0];

        for i in 0..n {
            let current_size = result.len();
            for j in (0..current_size).rev() {
                result.push(result[j] | 1 << i);
            }
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::gray_code(2)); // [0,1,3,2]

    println!("{:?}", Solution::gray_code(1)); // [0,1]
}

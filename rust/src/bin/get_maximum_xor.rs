// https://leetcode.com/problems/maximum-xor-for-each-query/
// 2024/11/08

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let max_xor = (1 << maximum_bit) - 1;

        nums.into_iter()
            .scan(0, |acc, x| {
                *acc ^= x;
                Some(*acc)
            })
            .map(|x| x ^ max_xor)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::get_maximum_xor([0, 1, 1, 3].to_vec(), 2)); // [0,3,2,3]

    println!("{:?}", Solution::get_maximum_xor([2, 3, 4, 7].to_vec(), 3)); // [5,2,6,5]

    println!(
        "{:?}",
        Solution::get_maximum_xor([0, 1, 2, 2, 5, 7].to_vec(), 3)
    ); // [4,3,6,4,6,7]
}

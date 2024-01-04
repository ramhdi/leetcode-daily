// https://leetcode.com/problems/check-if-bitwise-or-has-trailing-zeros/

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count: usize = 0;
        for num in nums {
            if num % 2 == 0 {
                count += 1;
            }
            if count >= 2 {
                return true;
            }
        }
        return false;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::has_trailing_zeros([1, 2, 3, 4, 5].to_vec())
    );
    println!("{:?}", Solution::has_trailing_zeros([2, 4, 8, 16].to_vec()));
    println!(
        "{:?}",
        Solution::has_trailing_zeros([1, 3, 5, 7, 9].to_vec())
    );
}

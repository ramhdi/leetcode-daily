// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array
// 2023/12/12

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut biggest = 0;
        let mut second_biggest = 0;
        for n in nums {
            if n > biggest {
                second_biggest = biggest;
                biggest = n;
            } else {
                second_biggest = std::cmp::max(second_biggest, n);
            }
        }

        return (biggest - 1) * (second_biggest - 1);
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::max_product(vec![1, 5, 4, 5]));
}

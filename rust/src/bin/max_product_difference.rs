// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
// 2023/12/18

impl Solution {
    pub fn _max_product_difference(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let nums_sorted: Vec<i32> = {
            let mut temp = nums;
            temp.sort();
            temp
        };
        return nums_sorted[len - 1] * nums_sorted[len - 2] - nums_sorted[1] * nums_sorted[0];
    }

    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut biggest: i32 = 0;
        let mut second_biggest: i32 = 0;
        let mut smallest: i32 = i32::MAX;
        let mut second_smallest: i32 = i32::MAX;

        for num in nums {
            if num > biggest {
                second_biggest = biggest;
                biggest = num;
            } else {
                second_biggest = std::cmp::max(second_biggest, num);
            }

            if num < smallest {
                second_smallest = smallest;
                smallest = num;
            } else {
                second_smallest = std::cmp::min(second_smallest, num);
            }
        }

        return biggest * second_biggest - smallest * second_smallest;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8])
    );
}

// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let nums_sorted: Vec<i32> = {
            let mut temp = nums;
            temp.sort();
            temp
        };

        let mut num_change: i32 = 0;
        for i in 0..nums_sorted.len() - 1 {
            if nums_sorted[i] < nums_sorted[i + 1] {
                num_change += 1;
            }
            result += num_change;
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::reduction_operations(vec![5, 1, 3]));
    println!("{}", Solution::reduction_operations(vec![1, 1, 1]));
    println!("{}", Solution::reduction_operations(vec![1, 1, 2, 2, 3]));
    println!("{}", Solution::reduction_operations(vec![1]));
    println!(
        "{}",
        Solution::reduction_operations(vec![7, 9, 10, 8, 6, 4, 1, 5, 2, 3])
    );
}

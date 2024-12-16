// https://leetcode.com/problems/continuous-subarrays/
// 2024/12/14

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        ((2..n).fold(0, |acc, window_size| {
            acc + nums.windows(window_size).fold(0, |acc2, subarray| {
                acc2 + subarray.windows(2).fold(0, |acc3, pair| {
                    if (pair[0] - pair[1]).abs() <= 2 {
                        acc3 + 1
                    } else {
                        acc3
                    }
                })
            })
        }) + n) as i64
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::continuous_subarrays([5, 4, 2, 4].to_vec())
    ); // 8

    println!("{:?}", Solution::continuous_subarrays([1, 2, 3].to_vec())); // 6
}

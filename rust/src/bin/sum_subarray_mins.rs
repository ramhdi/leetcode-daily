// https://leetcode.com/problems/sum-of-subarray-minimums/
// 2024/01/20

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        // keep a monotonic stack
        // for each element we add, we need to update suffix minimum
        // also need to keep track of the sum of values in the stack and update this
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut sum: i64 = 0;
        let mut result: i64 = 0;

        for (i, &x) in arr.iter().enumerate() {
            let mut last = i;
            while let Some(&(y, j)) = stack.last() {
                if x > y {
                    break;
                }

                stack.pop();
                sum -= (y - x) as i64 * (last - j) as i64;
                last = j;
            }

            stack.push((x, last));
            sum += x as i64;
            result = (result + sum) % 1_000_000_007;
        }

        return result as i32;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::sum_subarray_mins([3, 1, 2, 4].to_vec())); // 17
    println!(
        "{:?}",
        Solution::sum_subarray_mins([11, 81, 94, 43, 3].to_vec())
    ); // 444
}

// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/
// 2024/02/15

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();

        let mut sum: i64 = nums.iter().map(|&i| i as i64).sum();
        for &n in nums[2..].iter().rev() {
            let n = n as i64;
            sum -= n;
            if sum > n {
                return sum + n;
            }
        }

        return -1;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::largest_perimeter([5, 5, 5].to_vec())); // 15

    println!(
        "{:?}",
        Solution::largest_perimeter([1, 12, 1, 2, 5, 50, 3].to_vec())
    ); // 12

    println!("{:?}", Solution::largest_perimeter([5, 5, 50].to_vec())); // -1
}

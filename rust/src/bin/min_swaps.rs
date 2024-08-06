// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/?
// 2024/08/02

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let count_ones = nums.iter().filter(|&&x| x == 1).count();
        if count_ones == 0 || count_ones == n {
            return 0;
        }

        let mut count_zeros = nums[0..count_ones].iter().filter(|&&x| x == 0).count();
        let mut min_swap = count_zeros;

        for i in 0..n {
            if nums[i] == 0 {
                count_zeros -= 1;
            }
            if nums[(i + count_ones) % n] == 0 {
                count_zeros += 1;
            }
            min_swap = min_swap.min(count_zeros);
        }

        min_swap as i32
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::min_swaps([0, 1, 0, 1, 1, 0, 0].to_vec())); // 1

    println!(
        "{}",
        Solution::min_swaps([0, 1, 1, 1, 0, 0, 1, 1, 0].to_vec())
    ); // 2
}

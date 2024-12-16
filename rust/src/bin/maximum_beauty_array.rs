// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/
// 2024/12/10

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut max_beauty = 0;
        let mut i = 0;

        for j in 0..nums.len() {
            while nums[j] - nums[i] > 2 * k {
                i += 1;
            }

            max_beauty = max_beauty.max(j - i + 1);
        }

        max_beauty as i32
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::maximum_beauty([4, 6, 1, 2].to_vec(), 2)); // 3

    println!("{:?}", Solution::maximum_beauty([1, 1, 1, 1].to_vec(), 10)); // 4
}

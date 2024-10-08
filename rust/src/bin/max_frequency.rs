// https://leetcode.com/problems/frequency-of-the-most-frequent-element/

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let nums_sorted: Vec<i32> = {
            let mut temp = nums;
            temp.sort();
            temp
        };

        let mut result: i32 = 0;

        let mut left: usize = 0;
        let mut curr: u64 = 0;
        for right in 0..nums_sorted.len() {
            let target = nums_sorted[right];
            curr += target as u64;

            while (right - left + 1) as u64 * target as u64 - curr > k as u64 {
                curr -= nums_sorted[left] as u64;
                left += 1;
            }

            result = std::cmp::max(result, (right - left + 1) as i32);
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::max_frequency(vec![1, 2, 4], 5));
    println!("{:?}", Solution::max_frequency(vec![1, 4, 8, 13], 5));
    println!("{:?}", Solution::max_frequency(vec![3, 9, 6], 2));
    println!("{:?}", Solution::max_frequency(vec![1], 1));
}

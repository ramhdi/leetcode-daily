// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
// 2024/03/29

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut result: i64 = 0;
        let max_num = *nums.iter().max().unwrap();
        let n = nums.len();
        let mut count_max = 0;

        let mut i = 0;
        let mut j = 0;
        while j < n {
            if nums[j] == max_num {
                count_max += 1;
            }

            while count_max >= k {
                if nums[i] == max_num {
                    count_max -= 1;
                }
                i += 1;
                result += (n - j) as i64;
            }

            j += 1;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_subarrays([1, 3, 2, 3, 3].to_vec(), 2)
    ); // 6

    println!("{:?}", Solution::count_subarrays([1, 4, 2, 1].to_vec(), 3)); // 0
}

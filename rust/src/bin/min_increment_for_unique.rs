// https://leetcode.com/problems/minimum-increment-to-make-array-unique/
// 2024/06/14

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        // let mut unique: Vec<i32> = Vec::with_capacity(nums.len());
        nums.sort_unstable();

        let mut last_num: i32 = nums[0];
        // unique.push(last_num);
        for i in 1..nums.len() {
            let delta = nums[i] - last_num;
            if delta <= 0 {
                result += 1 - delta;
                last_num = nums[i] + 1 - delta;
            } else {
                last_num = nums[i];
            }
            // unique.push(last_num);
        }

        // println!("{:?}", unique);

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_increment_for_unique([1, 2, 2].to_vec())
    ); // 1

    println!(
        "{:?}",
        Solution::min_increment_for_unique([3, 2, 1, 2, 1, 7].to_vec())
    ); // 6

    println!("{:?}", Solution::min_increment_for_unique([1, 0].to_vec())); // 0
}

// https://leetcode.com/problems/set-mismatch/
// 2024/01/22

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut duplicate: i32 = 0;
        let mut missing: i32 = 0;
        let n = nums.len();
        let mut count: Vec<i32> = vec![0; n + 1];

        for num in nums {
            count[num as usize] += 1;
        }

        for i in 1..=n {
            if count[i] == 2 {
                duplicate = i as i32;
            } else if count[i] == 0 {
                missing = i as i32;
            }
        }

        return vec![duplicate, missing];
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_error_nums([1, 2, 2, 4].to_vec())); // [2,3]
    println!("{:?}", Solution::find_error_nums([1, 1].to_vec())); // [1,2]
    println!(
        "{:?}",
        Solution::find_error_nums([3, 2, 3, 4, 6, 5].to_vec())
    ); // [3,1]
}

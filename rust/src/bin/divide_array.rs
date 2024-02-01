// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/
// 2024/02/01

// impl Solution {
//     pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
//         nums.sort_unstable();
//         if nums.windows(3).step_by(3).all(|w| w[2] - w[0] <= k) {
//             return nums.windows(3).step_by(3).map(|w| w.to_vec()).collect();
//         }
//         return vec![];
//     }
// }

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        nums.sort_unstable();
        let n = nums.len() / 3;

        for i in 0..n {
            if nums[3 * i + 2] - nums[3 * i + 1] > k
                || nums[3 * i + 1] - nums[3 * i] > k
                || nums[3 * i + 2] - nums[3 * i] > k
            {
                return vec![];
            }
            result.push(vec![nums[3 * i], nums[3 * i + 1], nums[3 * i + 2]]);
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::divide_array([1, 3, 4, 8, 7, 9, 3, 5, 1].to_vec(), 2)
    ); // [[1,1,3],[3,4,5],[7,8,9]]

    println!(
        "{:?}",
        Solution::divide_array([1, 3, 3, 2, 7, 3].to_vec(), 3)
    ); // []]
}

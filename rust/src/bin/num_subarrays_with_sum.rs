// https://leetcode.com/problems/binary-subarrays-with-sum/
// 2024/03/14

// impl Solution {
//     pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
//         let sum = nums.clone().iter().sum::<i32>();
//         if sum < goal {
//             return 0;
//         }

//         let mut result: i32 = 0;
//         for start in 0..nums.len() {
//             for end in start..nums.len() {
//                 if nums[start..=end].iter().sum::<i32>() == goal {
//                     result += 1;
//                 }
//             }
//         }
//         return result;
//     }
// }

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        if goal == 0 {
            return count_subarrays_with_zero_sum(nums);
        } else {
            return count_subarrays_with_non_zero_sum(nums, goal);
        }
    }
}

// Function to count subarrays with a sum of zero
fn count_subarrays_with_zero_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cnt = 0;

    for &num in &nums {
        if num == 0 {
            cnt += 1;
        } else {
            ans += cnt * (cnt + 1) / 2;
            cnt = 0;
        }
    }

    ans += cnt * (cnt + 1) / 2; // Include the last sequence of zeros
    ans
}

// Function to count subarrays with a non-zero sum
fn count_subarrays_with_non_zero_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let (mut i, mut j, mut acc, mut ans) = (0, 0, 0, 0);

    while j < nums.len() {
        // Expand the window to the right until the sum reaches the goal or exceeds
        while j < nums.len() && acc < goal {
            acc += nums[j];
            j += 1;
        }

        // If the window sum does not match the goal, break out of the loop
        if acc != goal {
            break;
        }

        // Calculate the length of the continuous zero sequence to the right
        let mut right_zeros = 0;
        let mut right_index = j;
        while right_index < nums.len() && nums[right_index] == 0 {
            right_zeros += 1;
            right_index += 1;
        }

        // Calculate the length of the continuous zero sequence to the left
        let mut left_zeros = 0;
        while nums[i] == 0 {
            left_zeros += 1;
            i += 1;
        }

        // Count the combinations
        ans += (left_zeros + 1) * (right_zeros + 1);

        // Move the left pointer to the next non-zero element
        acc -= nums[i];
        i += 1;
    }

    ans
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::num_subarrays_with_sum([1, 0, 1, 0, 1].to_vec(), 2)
    ); // 4

    println!(
        "{:?}",
        Solution::num_subarrays_with_sum([0, 0, 0, 0, 0].to_vec(), 0)
    ); // 15
}

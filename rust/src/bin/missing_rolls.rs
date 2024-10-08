// https://leetcode.com/problems/find-missing-observations/
// 2024/09/05

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let missing_sum = mean * (n + rolls.len() as i32) - rolls.iter().sum::<i32>();
        if missing_sum < n || missing_sum > 6 * n {
            return vec![];
        }

        let mut result = vec![missing_sum / n; n as usize];
        let remainder = missing_sum % n;

        for i in 0..remainder as usize {
            result[i] += 1;
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::missing_rolls([3, 2, 4, 3].to_vec(), 4, 2)); // [6,6]

    println!("{:?}", Solution::missing_rolls([1, 5, 6].to_vec(), 3, 4)); // [2,3,2,2]

    println!("{:?}", Solution::missing_rolls([1, 2, 3, 4].to_vec(), 6, 4)); // []

    println!(
        "{:?}",
        Solution::missing_rolls(
            [
                4, 2, 2, 5, 4, 5, 4, 5, 3, 3, 6, 1, 2, 4, 2, 1, 6, 5, 4, 2, 3, 4, 2, 3, 3, 5, 4, 1,
                4, 4, 5, 3, 6, 1, 5, 2, 3, 3, 6, 1, 6, 4, 1, 3
            ]
            .to_vec(),
            2,
            53
        )
    ); // []
}

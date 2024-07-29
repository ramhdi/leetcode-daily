// https://leetcode.com/problems/count-number-of-teams/
// 2024/07/29

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();
        let mut result: i32 = 0;

        // decreasing
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                for k in j + 1..n {
                    if rating[i] > rating[j] && rating[j] > rating[k] {
                        result += 1;
                    }
                }
            }
        }

        // increasing
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                for k in j + 1..n {
                    if rating[i] < rating[j] && rating[j] < rating[k] {
                        result += 1;
                    }
                }
            }
        }

        return result;
    }
}
struct Solution {}

fn main() {
    println!("{:?}", Solution::num_teams([2, 5, 3, 4, 1].to_vec())); // 3

    println!("{:?}", Solution::num_teams([2, 1, 3].to_vec())); // 0

    println!("{:?}", Solution::num_teams([1, 2, 3, 4].to_vec())); // 4
}

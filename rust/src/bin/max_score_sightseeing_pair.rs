// https://leetcode.com/problems/best-sightseeing-pair/
// 2024/12/27

// Attempt 1: brute force, 1109ms, 2.50MB
// impl Solution {
//     pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
//         let n = values.len();
//         let mut result = 0;

//         for j in 1..n {
//             for i in 0..j {
//                 result = result.max(values[i] + values[j] + i as i32 - j as i32);
//             }
//         }

//         result
//     }
// }

// Attempt 2: dp, 0ms, 2.7MB
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max_left_score = values[0];
        let mut result = 0;

        for (i, score) in values.into_iter().skip(1).enumerate() {
            let j = (i + 1) as i32;
            result = result.max(max_left_score + score - j);
            max_left_score = max_left_score.max(score + j);
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_score_sightseeing_pair([8, 1, 5, 2, 6].to_vec())
    ); // 11

    println!(
        "{:?}",
        Solution::max_score_sightseeing_pair([1, 2].to_vec())
    ); // 2
}

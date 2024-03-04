// https://leetcode.com/problems/bag-of-tokens/
// 2024/03/04

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut result: i32 = 0;
        let mut score: i32 = 0;
        tokens.sort_unstable();

        let (mut i, mut j): (usize, usize) = (0, tokens.len() - 1);

        while i <= j {
            if power >= tokens[i] {
                power -= tokens[i];
                score += 1;
                result = result.max(score);
                i += 1;
            } else if score > 0 {
                power += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::bag_of_tokens_score([100].to_vec(), 50)); // 0

    println!(
        "{:?}",
        Solution::bag_of_tokens_score([200, 100].to_vec(), 150)
    ); // 1

    println!(
        "{:?}",
        Solution::bag_of_tokens_score([100, 200, 300, 400].to_vec(), 200)
    ); // 2
}

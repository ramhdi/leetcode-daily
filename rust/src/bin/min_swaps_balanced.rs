// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/
// 2024/10/08

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let (_, max_imbalance) = s.chars().fold((0, 0), |(mut b, mut mb), c| {
            if c == ']' {
                b -= 1;
            } else {
                b += 1;
            }

            if b < 0 {
                mb = mb.max(-b);
            }

            (b, mb)
        });

        (max_imbalance + 1) / 2
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_swaps("][][".to_string())); // 1

    println!("{:?}", Solution::min_swaps("]]][[[".to_string())); // 2

    println!("{:?}", Solution::min_swaps("[]".to_string())); // 0

    println!(
        "{:?}",
        Solution::min_swaps("[[[]]]][][]][[]]][[[".to_string())
    ); // 2
}

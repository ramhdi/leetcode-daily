// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/
// 2024/10/19

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let nth = Self::nth_string(n);
        println!("{:?}", nth);
        nth.chars().nth(k as usize - 1).unwrap()
    }

    fn nth_string(n: i32) -> String {
        if n <= 1 {
            return "0".to_string();
        }

        let prev = Self::nth_string(n - 1);
        prev.clone()
            + "1"
            + &prev
                .chars()
                .map(|c| if c == '0' { '1' } else { '0' })
                .rev()
                .collect::<String>()
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_kth_bit(3, 1)); // "0"

    println!("{:?}", Solution::find_kth_bit(4, 11)); // "1"

    println!("{:?}", Solution::find_kth_bit(3, 2)); // "1"
}

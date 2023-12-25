// https://leetcode.com/problems/decode-ways/
// 2023/12/25

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s == "0" || s.chars().nth(0).unwrap() == '0' {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        if s.len() == 2 {
            let s_parsed = s.parse::<i32>().unwrap();
            if s_parsed > 10 && s_parsed <= 26 {
                return 2;
            } else {
                return 1;
            }
        }
        let mut result: i32 = 0;
        result += Solution::num_decodings(s[1..].to_string());
        if s[0..2].to_string().parse::<i32>().unwrap() <= 26 {
            result += Solution::num_decodings(s[2..].to_string());
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::num_decodings("12".to_string()));
    println!("{}", Solution::num_decodings("226".to_string()));
    println!("{}", Solution::num_decodings("06".to_string()));
    println!("{}", Solution::num_decodings("2101".to_string()));
    println!("{}", Solution::num_decodings("123123".to_string()));
    println!(
        "{}",
        Solution::num_decodings("111111111111111111111111111111111111111111111".to_string())
    );
}

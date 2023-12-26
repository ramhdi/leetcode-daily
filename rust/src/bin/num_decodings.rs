// https://leetcode.com/problems/decode-ways/
// 2023/12/25

use std::collections::HashMap;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp: HashMap<String, i32> = HashMap::new();

        fn dp_helper(s: String, dp: &mut HashMap<String, i32>) -> i32 {
            if s == "0" || s.chars().nth(0).unwrap() == '0' {
                return 0;
            }
            if s.len() == 1 {
                return 1;
            }
            if s.len() == 2 {
                let s_parsed = s.parse::<i32>().unwrap();
                if s_parsed > 10 && s_parsed <= 26 && s_parsed % 10 != 0 {
                    return 2;
                } else if s_parsed == 10 || s_parsed == 20 || (s_parsed >= 26 && s_parsed % 10 != 0)
                {
                    return 1;
                } else {
                    return 0;
                }
            }
            if let Some(res) = dp.get(&s) {
                return *res;
            }
            let mut result: i32 = 0;

            let trailing_num = s[1..2].to_string().parse::<i32>().unwrap();
            let two_trailing_nums = s[1..3].to_string().parse::<i32>().unwrap();
            if (s.len() > 3 && trailing_num != 0)
                || (s.len() == 3 && !(two_trailing_nums >= 30 && two_trailing_nums % 10 == 0))
            {
                result += dp_helper(s[1..].to_string(), dp);
            }
            if s[0..2].to_string().parse::<i32>().unwrap() <= 26 {
                result += dp_helper(s[2..].to_string(), dp);
            }
            dp.insert(s, result);
            return result;
        }

        for i in s.len() - 1..0 {
            dp_helper(s[i..].to_string(), &mut dp);
        }
        return dp_helper(s, &mut dp);
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

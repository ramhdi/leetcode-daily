// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
// 2023/12/22

impl Solution {
    pub fn _max_score(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut sum_0: i32 = 0;
        let mut prefix_sum_0: Vec<i32> = Vec::new();
        let mut sum_1: i32 = 0;
        let mut suffix_sum_1: Vec<i32> = Vec::new();
        let s_rev: String = s.chars().rev().collect();

        for (s, r) in s.chars().zip(s_rev.chars()) {
            if s == '0' {
                sum_0 += 1;
            }
            prefix_sum_0.push(sum_0);

            if r == '1' {
                sum_1 += 1;
            }
            suffix_sum_1.push(sum_1);
        }

        prefix_sum_0.pop().unwrap();
        suffix_sum_1.pop().unwrap();
        suffix_sum_1.reverse();

        for (sz, so) in prefix_sum_0.iter().zip(suffix_sum_1.iter()) {
            result = result.max(sz + so);
        }

        return result;
    }

    pub fn max_score(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut ones: i32 = s.chars().filter(|&c| c == '1').count() as i32;
        let mut zeros: i32 = 0;
        let s2 = &s[0..s.len() - 1].to_string();
        for c in s2.chars() {
            match c {
                '0' => zeros += 1,
                '1' => ones -= 1,
                _ => (),
            }
            result = result.max(zeros + ones);
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::max_score("011101".to_string()));
    println!("{}", Solution::max_score("11".to_string()));
    println!("{}", Solution::max_score("00".to_string()));
    println!("{}", Solution::max_score("01".to_string()));
}

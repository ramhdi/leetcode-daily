// https://leetcode.com/problems/construct-k-palindrome-strings/

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut char_count: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let mut odd_count: usize = 0;
        for (_, value) in char_count.into_iter() {
            if value % 2 == 1 {
                odd_count += 1;
            }
        }
        if odd_count > k as usize {
            return false;
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::can_construct("annabelle".to_string(), 2));
    println!("{:?}", Solution::can_construct("leetcode".to_string(), 3));
    println!("{:?}", Solution::can_construct("true".to_string(), 4));
}

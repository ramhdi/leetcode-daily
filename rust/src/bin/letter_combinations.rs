// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits == "" {
            return vec![];
        }
        let mut result: Vec<String> = vec!["".to_string()];
        let num_to_chars: Vec<&str> =
            vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        for digit_char in digits.chars() {
            let digit_idx: usize = digit_char.to_digit(10).unwrap() as usize - 2;
            let mut temp: Vec<String> = Vec::new();

            for s in result {
                for c in num_to_chars[digit_idx].chars() {
                    temp.push(s.clone() + &c.to_string());
                }
            }

            result = temp;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}

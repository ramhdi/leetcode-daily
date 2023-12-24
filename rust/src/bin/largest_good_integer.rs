// https://leetcode.com/problems/largest-3-same-digit-number-in-string/

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut result: char = '0';
        let mut found: bool = false;

        for i in 0..num.len() - 2 {
            if num.chars().nth(i).unwrap() == num.chars().nth(i + 1).unwrap()
                && num.chars().nth(i).unwrap() == num.chars().nth(i + 2).unwrap()
                && num.chars().nth(i).unwrap() >= result
            {
                found = true;
                result = num.chars().nth(i).unwrap();
            }
        }

        if !found {
            return "".to_string();
        }

        return std::iter::repeat(result).take(3).collect();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::largest_good_integer("6777133339".to_string())
    );
    println!("{}", Solution::largest_good_integer("2300019".to_string()));
    println!("{}", Solution::largest_good_integer("42352338".to_string()));
    println!("{}", Solution::largest_good_integer("111".to_string()));
    println!("{}", Solution::largest_good_integer("121".to_string()));
}

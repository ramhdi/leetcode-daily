// https://leetcode.com/problems/make-the-string-great/
// 2024/04/05

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            let last = stack.last();

            if last.is_some() {
                let &l = last.unwrap();

                if (l as i8 - c as i8).abs() == 32 {
                    stack.pop();
                    continue;
                }
            }

            stack.push(c);
        }

        return String::from_iter(stack);
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::make_good("leEeetcode".to_string())); // leetcode
    println!("{:?}", Solution::make_good("abBAcC".to_string())); //
    println!("{:?}", Solution::make_good("s".to_string())); // s
}

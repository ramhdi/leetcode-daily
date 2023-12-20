// https://leetcode.com/problems/generate-parentheses/

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec!["(".to_string()];

        while result[0].len() < (n * 2) as usize {
            let mut temp: Vec<String> = Vec::new();

            for s in result {
                let open = s.chars().filter(|&c| c == '(').count() as i32;
                let close = s.chars().filter(|&c| c == ')').count() as i32;
                if open == n {
                    temp.push(s.clone() + ")");
                } else if open == close {
                    temp.push(s.clone() + "(");
                } else {
                    temp.push(s.clone() + "(");
                    temp.push(s.clone() + ")");
                }
            }
            result = temp;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}

// https://leetcode.com/problems/longest-valid-parentheses/

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![-1];
        let mut result: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    stack.push(i as i32);
                }
                ')' => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                    if !stack.is_empty() {
                        result = std::cmp::max(result, i as i32 - stack.last().unwrap());
                    } else {
                        stack.push(i as i32);
                    }
                }
                _ => (),
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("()(()".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses(")()())".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses("(()".to_string())
    );
    println!(
        "{:?}",
        Solution::longest_valid_parentheses(")(()(()(((())(((((()()))((((()()(()()())())())()))()()()())(())()()(((()))))()((()))(((())()((()()())((())))(())))())((()())()()((()((())))))((()(((((()((()))(()()(())))((()))()))())".to_string())
    );
}

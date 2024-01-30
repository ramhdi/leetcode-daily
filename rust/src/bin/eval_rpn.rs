// https://leetcode.com/problems/evaluate-reverse-polish-notation/
// 2024/01/30

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            let n = stack.len();
            match token.as_str() {
                "+" => {
                    stack[n - 2] = stack[n - 2] + stack[n - 1];
                    stack.pop();
                }
                "-" => {
                    stack[n - 2] = stack[n - 2] - stack[n - 1];
                    stack.pop();
                }
                "*" => {
                    stack[n - 2] = stack[n - 2] * stack[n - 1];
                    stack.pop();
                }
                "/" => {
                    stack[n - 2] = stack[n - 2] / stack[n - 1];
                    stack.pop();
                }
                num_str => {
                    if let Ok(num) = num_str.parse() {
                        stack.push(num);
                    }
                }
            }
        }

        return stack[0];
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::eval_rpn(["2", "1", "+", "3", "*"].map(|s| s.to_string()).to_vec())
    ); // 9

    println!(
        "{:?}",
        Solution::eval_rpn(["4", "13", "5", "/", "+"].map(|s| s.to_string()).to_vec())
    ); // 6

    println!(
        "{:?}",
        Solution::eval_rpn(
            ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                .map(|s| s.to_string())
                .to_vec()
        )
    ); // 22
}

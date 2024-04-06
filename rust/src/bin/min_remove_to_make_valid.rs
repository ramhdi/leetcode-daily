// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
// 2024/04/06

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        let mut n = 0;

        for c in s.chars() {
            if n == 0 && c == ')' {
                continue;
            }

            if c == '(' {
                n += 1;
            }

            if c == ')' {
                n -= 1;
            }

            stack.push(c);
        }

        let mut stack2: Vec<char> = Vec::with_capacity(s.len());
        n = 0;
        for &c in stack.iter().rev() {
            if n == 0 && c == '(' {
                continue;
            }

            if c == ')' {
                n += 1;
            }

            if c == '(' {
                n -= 1;
            }

            stack2.push(c);
        }

        return String::from_iter(stack2.iter().rev());
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string())
    ); // lee(t(c)o)de

    println!(
        "{:?}",
        Solution::min_remove_to_make_valid("a)b(c)d".to_string())
    ); // ab(c)d

    println!(
        "{:?}",
        Solution::min_remove_to_make_valid("))((".to_string())
    ); //
}

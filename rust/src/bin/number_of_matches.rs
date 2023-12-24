// https://leetcode.com/problems/count-of-matches-in-tournament/
// 2023/12/24

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        return n - 1;
    }

    pub fn _number_of_matches(mut n: i32) -> i32 {
        if n <= 2 {
            return n - 1;
        }
        let mut result: i32 = 0;

        while n >= 2 {
            match n % 2 {
                0 => {
                    result += n / 2;
                    n /= 2;
                }
                1 => {
                    result += n / 2;
                    n = n / 2 + 1;
                }
                _ => (),
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::number_of_matches(7));
    println!("{}", Solution::number_of_matches(14));
}

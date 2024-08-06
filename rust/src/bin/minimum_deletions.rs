// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
// 2024/07/30

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut count_b: i32 = 0;
        let mut count_a: i32 = s.chars().filter(|&c| c == 'a').count() as i32;
        let mut result: i32 = count_a;

        for c in s.chars() {
            match c {
                'a' => {
                    count_a -= 1;
                    result = std::cmp::min(result, count_b + count_a);
                }
                'b' => {
                    count_b += 1;
                }
                _ => (),
            }
        }

        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::minimum_deletions("aababbab".to_string())); // 2

    println!("{}", Solution::minimum_deletions("bbaaaaabb".to_string())); // 2
}

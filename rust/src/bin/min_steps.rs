// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
// 2024/01/13

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut result: i32 = 0;
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut count_s: Vec<i32> = vec![0; 26];
        let mut count_t: Vec<i32> = vec![0; 26];
        let n = s.len();

        for i in 0..n {
            count_s[s[i] as usize - 'a' as usize] += 1;
            count_t[t[i] as usize - 'a' as usize] += 1;
        }

        for i in 0..26 {
            if count_t[i] < count_s[i] {
                result += count_s[i] - count_t[i];
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_steps("bab".to_string(), "aba".to_string())
    ); // 1
    println!(
        "{:?}",
        Solution::min_steps("leetcode".to_string(), "practice".to_string())
    ); // 5
    println!(
        "{:?}",
        Solution::min_steps("anagram".to_string(), "mangaar".to_string())
    ); // 5
}

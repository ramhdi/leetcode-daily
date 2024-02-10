// https://leetcode.com/problems/palindromic-substrings/
// 2024/02/10

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut result: i32 = 0;

        for start in (0..n).rev() {
            for end in start..n {
                if s[start] == s[end] {
                    let len = end - start + 1;
                    if len <= 3 {
                        dp[start][end] = true;
                    } else {
                        dp[start][end] = dp[start + 1][end - 1];
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                result += dp[i][j] as i32;
            }
        }

        return result;
    }
}

// impl Solution {
//     pub fn count_substrings(s: String) -> i32 {
//         let s = s.as_bytes();
//         (0..s.len())
//             .map(|i| Self::expand(&s, i, i) + Self::expand(&s, i, i + 1))
//             .sum()
//     }

//     #[inline]
//     fn expand(s: &[u8], mut l: usize, mut r: usize) -> i32 {
//         let mut count = 0;
//         while l < s.len() && r < s.len() && s[l] == s[r] {
//             count += 1;
//             if l == 0 {
//                 break;
//             }
//             l -= 1;
//             r += 1;
//         }
//         count
//     }
// }

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::count_substrings("abc".to_string())); // 3
    println!("{:?}", Solution::count_substrings("aaa".to_string())); // 6
    println!("{:?}", Solution::count_substrings("aaaaa".to_string())); // 15
}

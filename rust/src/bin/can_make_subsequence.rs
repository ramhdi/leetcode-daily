// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/
// 2024/12/04

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let (b1, b2) = (str1.as_bytes(), str2.as_bytes());
        let (m, n) = (b1.len(), b2.len());
        let (mut i, mut j) = (0usize, 0usize);

        while i < m && j < n {
            if b1[i] == b2[j] || Self::increment_char(b1[i]) == b2[j] {
                j += 1;
            }
            i += 1;
        }

        j == n
    }

    fn increment_char(input: u8) -> u8 {
        ((input - b'a' + 1) % 26) + b'a'
    }
}

// impl Solution {
//     pub fn can_make_subsequence(str1: String, str2: String) -> bool {
//         Self::can_make_subsequence_helper(str1.as_bytes(), str2.as_bytes(), 0, 0)
//     }

//     fn can_make_subsequence_helper(b1: &[u8], b2: &[u8], i: usize, j: usize) -> bool {
//         if j == b2.len() {
//             return true;
//         }

//         if i == b1.len() {
//             return false;
//         }

//         if b1[i] == b2[j] || Self::increment_char(b1[i]) == b2[j] {
//             return Self::can_make_subsequence_helper(b1, b2, i + 1, j + 1);
//         }

//         Self::can_make_subsequence_helper(b1, b2, i + 1, j)
//     }

//     fn increment_char(input: u8) -> u8 {
//         ((input - b'a' + 1) % 26) + b'a'
//     }
// }

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::can_make_subsequence("abc".to_string(), "ad".to_string())
    ); // true

    println!(
        "{:?}",
        Solution::can_make_subsequence("zc".to_string(), "ad".to_string())
    ); // true

    println!(
        "{:?}",
        Solution::can_make_subsequence("ab".to_string(), "d".to_string())
    ); // false
}

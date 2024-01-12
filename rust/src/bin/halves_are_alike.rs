// https://leetcode.com/problems/determine-if-string-halves-are-alike/
// 2024/01/12

impl Solution {
    pub fn count_vowel(s: &str) -> usize {
        let mut result: usize = 0;
        for c in s.chars() {
            if c == 'a'
                || c == 'e'
                || c == 'i'
                || c == 'o'
                || c == 'u'
                || c == 'A'
                || c == 'E'
                || c == 'I'
                || c == 'O'
                || c == 'U'
            {
                result += 1;
            }
        }
        return result;
    }

    pub fn halves_are_alike(s: String) -> bool {
        let n = s.len();
        return Solution::count_vowel(&s[..n / 2]) == Solution::count_vowel(&s[n / 2..]);
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::halves_are_alike("book".to_string())); // true
    println!("{:?}", Solution::halves_are_alike("textbook".to_string())); // false
}

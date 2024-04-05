// https://leetcode.com/problems/isomorphic-strings/
// 2024/04/02

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_s: [usize; 128] = [0; 128];
        let mut count_t: [usize; 128] = [0; 128];

        for (i, (cs, ct)) in s.bytes().zip(t.bytes()).enumerate() {
            if count_s[cs as usize] != count_t[ct as usize] {
                return false;
            }

            count_s[cs as usize] = i + 1;
            count_t[ct as usize] = i + 1;
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_isomorphic("egg".to_string(), "add".to_string())
    ); // true
    println!(
        "{:?}",
        Solution::is_isomorphic("foo".to_string(), "bar".to_string())
    ); // false
    println!(
        "{:?}",
        Solution::is_isomorphic("paper".to_string(), "title".to_string())
    ); // true
}

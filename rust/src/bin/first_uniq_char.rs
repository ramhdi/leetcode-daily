// https://leetcode.com/problems/first-unique-character-in-a-string/
// 2024/02/05

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_count = [0 as u32; (b'z' - b'a' + 1) as usize];
        let s = s.as_bytes();

        for &c in s {
            char_count[(c - b'a') as usize] += 1;
        }

        for (i, &c) in s.iter().enumerate() {
            if char_count[(c - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        return -1;
    }
}
pub struct Solution {}

fn main() {
    println!("{:?}", Solution::first_uniq_char("leetcode".to_string())); // 0
    println!(
        "{:?}",
        Solution::first_uniq_char("loveleetcode".to_string())
    ); // 2
    println!("{:?}", Solution::first_uniq_char("aabb".to_string())); // -1
}

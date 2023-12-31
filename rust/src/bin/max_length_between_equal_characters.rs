// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
// 2023/12/31

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut result = -1;
        let mut char_indices = vec![-1; 26];
        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            if char_indices[index] == -1 {
                char_indices[index] = i as i32;
            } else {
                result = result.max(i as i32 - char_indices[index] - 1);
            }
        }

        return result;
    }

    pub fn _max_length_between_equal_characters(s: String) -> i32 {
        let mut result: i32 = -1;
        let mut char_indexes: Vec<Vec<usize>> = vec![vec![]; 26];
        for (i, ch) in s.chars().enumerate() {
            char_indexes[ch as usize - 'a' as usize].push(i);
        }

        for indexes in char_indexes {
            let n = indexes.len();
            if n >= 2 {
                result = std::cmp::max(result, (indexes[n - 1] - indexes[0] - 1) as i32);
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_length_between_equal_characters("aa".to_string())
    );
    println!(
        "{:?}",
        Solution::max_length_between_equal_characters("abca".to_string())
    );
    println!(
        "{:?}",
        Solution::max_length_between_equal_characters("cbzxy".to_string())
    );
}

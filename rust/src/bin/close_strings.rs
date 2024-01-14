// https://leetcode.com/problems/determine-if-two-strings-are-close/
// 2024/01/14

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut chars1: [bool; 26] = [false; 26];
        let mut counts1: [usize; 26] = [0; 26];
        let mut chars2: [bool; 26] = [false; 26];
        let mut counts2: [usize; 26] = [0; 26];

        for (ch1, ch2) in word1.chars().zip(word2.chars()) {
            let char_num1 = ch1 as usize - 'a' as usize;
            chars1[char_num1] = true;
            counts1[char_num1] += 1;

            let char_num2 = ch2 as usize - 'a' as usize;
            chars2[char_num2] = true;
            counts2[char_num2] += 1;
        }

        for i in 0..26 {
            if chars1[i] != chars2[i] {
                return false;
            }
        }

        counts1.sort_unstable();
        counts2.sort_unstable();

        return counts1 == counts2;
    }

    pub fn _close_strings(word1: String, word2: String) -> bool {
        let mut map1: HashMap<char, usize> = HashMap::new();
        let mut map2: HashMap<char, usize> = HashMap::new();

        for c in word1.chars() {
            *map1.entry(c).or_insert(0) += 1;
        }

        for c in word2.chars() {
            *map2.entry(c).or_insert(0) += 1;
        }

        let mut set1: HashSet<char> = HashSet::new();
        let mut count1: Vec<usize> = Vec::new();
        let mut set2: HashSet<char> = HashSet::new();
        let mut count2: Vec<usize> = Vec::new();

        for (k, v) in map1 {
            set1.insert(k);
            count1.push(v);
        }

        for (k, v) in map2 {
            set2.insert(k);
            count2.push(v);
        }

        count1.sort_unstable();
        count2.sort_unstable();

        return set1 == set2 && count1 == count2;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::close_strings("abc".to_string(), "bca".to_string())
    ); // true
    println!(
        "{:?}",
        Solution::close_strings("a".to_string(), "aa".to_string())
    ); // false
    println!(
        "{:?}",
        Solution::close_strings("cabbba".to_string(), "abbccc".to_string())
    ); // true
    println!(
        "{:?}",
        Solution::close_strings(
            "aaabbbbccddeeeeefffff".to_string(),
            "aaaaabbcccdddeeeeffff".to_string()
        )
    ); // false
}

// https://leetcode.com/problems/short-encoding-of-words/

use std::collections::HashSet;

impl Solution {
    pub fn is_end_subset(word1: String, word2: String) -> bool {
        if word1 == word2 {
            return true;
        }

        let n1 = word1.len();
        let n2 = word2.len();
        let mut i: usize = 1;
        while i <= n2 {
            if word1.chars().nth(n1 - i).unwrap() != word2.chars().nth(n2 - i).unwrap() {
                return false;
            } else {
                i += 1;
            }
        }
        return true;
    }

    pub fn _minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut result: i32 = 0;

        let words_sorted: Vec<String> = {
            let mut temp = words.clone();
            temp.sort_by(|a, b| b.len().cmp(&a.len()));
            temp
        };

        let mut words_set: HashSet<String> = HashSet::new();
        for word in words_sorted {
            let mut found = false;
            for w in words_set.iter() {
                found = found || Solution::is_end_subset(w.clone(), word.clone());
            }
            if !found {
                words_set.insert(word);
            }
        }

        for w in words_set.iter() {
            result += w.len() as i32;
        }
        result += words_set.len() as i32;

        return result;
    }

    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut result: i32 = 0;
        let mut words_set: HashSet<String> = words.clone().into_iter().collect();

        for word in words {
            for i in 1..word.len() {
                words_set.remove(&word[i..].to_string());
            }
        }

        for w in words_set {
            result += (w.len() + 1) as i32;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::minimum_length_encoding(vec!["feipyxx".to_string(), "e".to_string()])
    );
}

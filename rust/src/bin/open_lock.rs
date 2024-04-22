// https://leetcode.com/problems/open-the-lock/
// 2024/04/22

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadends: HashSet<String> = deadends.into_iter().collect();
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        let start: String = "0000".to_string();

        if deadends.contains(&start) {
            return -1;
        }

        queue.push_back((start, 0));

        while let Some((current, steps)) = queue.pop_front() {
            if current == target {
                return steps;
            }

            for i in 0..4 {
                let chars: Vec<char> = current.chars().collect();
                for d in -1..=1 {
                    if d == 0 {
                        continue;
                    }

                    let mut new_chars: Vec<char> = chars.clone();
                    let new_digit: u8 =
                        (((new_chars[i] as u8 - b'0') as i32 + d + 10) % 10) as u8 + b'0';
                    new_chars[i] = new_digit as char;
                    let new_combination: String = new_chars.iter().collect();

                    if !deadends.contains(&new_combination) {
                        queue.push_back((new_combination.clone(), steps + 1));
                        deadends.insert(new_combination); // Visited node should not be visited again
                    }
                }
            }
        }

        return -1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::open_lock(
            ["0201", "0101", "0102", "1212", "2002"]
                .map(|e| e.to_string())
                .to_vec(),
            "0202".to_string(),
        )
    ); // 6

    println!(
        "{:?}",
        Solution::open_lock(["8888"].map(|e| e.to_string()).to_vec(), "0009".to_string(),)
    ); // 1

    println!(
        "{:?}",
        Solution::open_lock(
            ["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]
                .map(|e| e.to_string())
                .to_vec(),
            "8888".to_string(),
        )
    ); // -1
}

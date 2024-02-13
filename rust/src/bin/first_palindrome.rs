// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// 2024/02/13

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            let reversed = word.clone().chars().rev().collect::<String>();
            if word == reversed {
                return word;
            }
        }

        return String::new();
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::first_palindrome(
            ["abc", "car", "ada", "racecar", "cool"]
                .map(|w| w.to_string())
                .to_vec()
        )
    ); // ada

    println!(
        "{:?}",
        Solution::first_palindrome(
            ["notapalindrome", "racecar"]
                .map(|w| w.to_string())
                .to_vec()
        )
    ); // racecar

    println!(
        "{:?}",
        Solution::first_palindrome(["def", "ghi"].map(|w| w.to_string()).to_vec())
    ); // ""
}

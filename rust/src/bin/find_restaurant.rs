// https://leetcode.com/problems/minimum-index-sum-of-two-lists/

use std::collections::HashSet;

impl Solution {
    pub fn find_restaurant(mut list1: Vec<String>, mut list2: Vec<String>) -> Vec<String> {
        let mut set1: HashSet<String> = HashSet::new();
        let mut set2: HashSet<String> = HashSet::new();
        let mut result: Vec<String> = Vec::new();
        let mut s1: String = String::new();
        let mut s2: String = String::new();
        while list1.len() > 0 || list2.len() > 0 {
            if list1.len() > 0 {
                s1 = list1.pop().unwrap();
                set1.insert(s1.clone());
            }
            if set2.contains(&s1) {
                result.push(s1.clone());
            }
            if list2.len() > 0 {
                s2 = list2.pop().unwrap();
                set2.insert(s2.clone());
            }
            if set1.contains(&s2) {
                result.push(s2.clone());
            }
            if result.len() > 0 {
                return result;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_restaurant(
            ["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .map(|s| s.to_string())
                .to_vec(),
            [
                "Piatti",
                "The Grill at Torrey Pines",
                "Hungry Hunter Steakhouse",
                "Shogun"
            ]
            .map(|s| s.to_string())
            .to_vec()
        )
    );
    println!(
        "{:?}",
        Solution::find_restaurant(
            ["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .map(|s| s.to_string())
                .to_vec(),
            ["KFC", "Shogun", "Burger King"]
                .map(|s| s.to_string())
                .to_vec()
        )
    );
    println!(
        "{:?}",
        Solution::find_restaurant(
            ["happy", "sad", "good"].map(|s| s.to_string()).to_vec(),
            ["sad", "happy", "good"].map(|s| s.to_string()).to_vec()
        )
    );
}

// https://leetcode.com/problems/kth-distinct-string-in-an-array/
// 2024/08/05

use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut str_count: HashMap<&String, i32> = HashMap::new();

        for str in &arr {
            *str_count.entry(str).or_insert(0) += 1;
        }

        let mut counter: i32 = 0;
        for str in &arr {
            match str_count.get(str) {
                Some(1) => {
                    counter += 1;
                    if counter == k {
                        return str.to_string();
                    }
                }
                _ => {}
            }
        }

        String::new()
    }
}

struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::kth_distinct(
            ["d", "b", "c", "b", "c", "a"]
                .map(|s| s.to_string())
                .to_vec(),
            2
        )
    ); // "a"

    println!(
        "{}",
        Solution::kth_distinct(["aaa", "aa", "a"].map(|s| s.to_string()).to_vec(), 1)
    ); // "aaa"

    println!(
        "{}",
        Solution::kth_distinct(["a", "b", "a"].map(|s| s.to_string()).to_vec(), 3)
    ); // ""
}

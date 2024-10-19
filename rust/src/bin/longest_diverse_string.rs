// https://leetcode.com/problems/longest-happy-string/
// 2024/10/16

use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut result = String::new();
        let mut heap = BinaryHeap::new();

        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        while let Some((mut count1, ch1)) = heap.pop() {
            if result.len() >= 2 && result.ends_with(&ch1.to_string().repeat(2)) {
                if let Some((mut count2, ch2)) = heap.pop() {
                    result.push(ch2);
                    count2 -= 1;
                    if count2 > 0 {
                        heap.push((count2, ch2));
                    }
                    heap.push((count1, ch1)); // push back the first character
                } else {
                    break; // no valid second character to use, break
                }
            } else {
                let limit = std::cmp::min(2, count1); // add at most 2 characters
                for _ in 0..limit {
                    result.push(ch1);
                }
                count1 -= limit;
                if count1 > 0 {
                    heap.push((count1, ch1)); // push back the character with reduced count
                }
            }
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::longest_diverse_string(1, 1, 7)); // "ccaccbcc"

    println!("{:?}", Solution::longest_diverse_string(7, 1, 0)); // "aabaa"
}

// https://leetcode.com/problems/construct-string-with-repeat-limit/
// 2024/12/17

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut pq: BinaryHeap<(char, i32)> = s
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect();

        let mut result = String::new();
        while let Some((curr, mut count_curr)) = pq.pop() {
            let repeat = std::cmp::min(count_curr, repeat_limit);
            result.extend(std::iter::repeat(curr).take(repeat as usize));

            count_curr -= repeat;
            if count_curr > 0 {
                if let Some((next, mut count_next)) = pq.pop() {
                    result.push(next);
                    count_next -= 1;

                    if count_next > 0 {
                        pq.push((next, count_next));
                    }
                } else {
                    return result;
                }

                pq.push((curr, count_curr));
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::repeat_limited_string("cczazcc".to_string(), 3)
    ); // "zzcccac"

    println!(
        "{:?}",
        Solution::repeat_limited_string("aababab".to_string(), 2)
    ); // "bbabaa"
}

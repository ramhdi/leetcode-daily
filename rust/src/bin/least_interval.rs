// https://leetcode.com/problems/task-scheduler/
// 2024/03/19

use std::collections::HashMap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        let len = tasks.len();
        for task in tasks {
            *freq_map.entry(task).or_insert(0) += 1;
        }

        let mut tasks_freq: Vec<(char, i32)> = freq_map.into_iter().collect();
        tasks_freq.sort_by_key(|&(_, freq)| std::cmp::Reverse(freq));

        let max_freq = tasks_freq[0].1;
        let mut idle_slots = (max_freq - 1) * n;

        for i in 1..tasks_freq.len() {
            let (_, freq) = tasks_freq[i];
            idle_slots -= std::cmp::min(max_freq - 1, freq);
        }

        let idle_slots = std::cmp::max(0, idle_slots);

        return (len as i32) + idle_slots;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::least_interval(
            ["A", "A", "A", "B", "B", "B"]
                .map(|s| s.chars().nth(0).unwrap())
                .to_vec(),
            2
        )
    ); // 8

    println!(
        "{:?}",
        Solution::least_interval(
            ["A", "C", "A", "B", "D", "B"]
                .map(|s| s.chars().nth(0).unwrap())
                .to_vec(),
            1
        )
    ); // 6

    println!(
        "{:?}",
        Solution::least_interval(
            ["A", "A", "A", "B", "B", "B"]
                .map(|s| s.chars().nth(0).unwrap())
                .to_vec(),
            3
        )
    ); // 10
}

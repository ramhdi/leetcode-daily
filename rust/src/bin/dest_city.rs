// https://leetcode.com/problems/destination-city/
// 2023/12/15

use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut result: String = paths[0][0].clone();
        let from_to: HashMap<String, String> = paths
            .into_iter()
            .map(|pair| (pair[0].clone(), pair[1].clone()))
            .collect();

        while let Some(destination) = from_to.get(&result) {
            result = destination.to_owned();
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::dest_city(vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()]
        ])
    );
}

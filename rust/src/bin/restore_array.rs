// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacency_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for pair in &adjacent_pairs {
            adjacency_map.entry(pair[0]).or_default().push(pair[1]);
            adjacency_map.entry(pair[1]).or_default().push(pair[0]);
        }

        let mut result: Vec<i32> = Vec::new();
        let mut visited: HashSet<i32> = HashSet::new();

        let mut start = 0;
        for (num, neighbors) in &adjacency_map {
            if neighbors.len() == 1 {
                start = *num;
                break;
            }
        }

        Solution::dfs(start, &adjacency_map, &mut visited, &mut result);

        result
    }

    fn dfs(
        node: i32,
        adjacency_map: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
        result: &mut Vec<i32>,
    ) {
        if visited.contains(&node) {
            return;
        }

        visited.insert(node);
        result.push(node);

        if let Some(neighbors) = adjacency_map.get(&node) {
            for &neighbor in neighbors {
                Solution::dfs(neighbor, adjacency_map, visited, result);
            }
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::restore_array([[2, 1], [3, 4], [3, 2]].map(|e| e.to_vec()).to_vec())
    ); // [1,2,3,4]

    println!(
        "{:?}",
        Solution::restore_array([[2, 1], [3, 4], [3, 2]].map(|e| e.to_vec()).to_vec())
    ); // [-2,4,1,-3]

    println!(
        "{:?}",
        Solution::restore_array([[100000, -100000]].map(|e| e.to_vec()).to_vec())
    ); // [100000,-100000]
}

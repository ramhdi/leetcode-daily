// https://leetcode.com/problems/valid-arrangement-of-pairs/
// 2024/11/30

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Build the adjacency list and in/out degree counts
        let mut graph = HashMap::new();
        let mut out_degree = HashMap::new();
        let mut in_degree = HashMap::new();

        for pair in &pairs {
            let (start, end) = (pair[0], pair[1]);
            graph.entry(start).or_insert_with(Vec::new).push(end);
            *out_degree.entry(start).or_insert(0) += 1;
            *in_degree.entry(end).or_insert(0) += 1;
        }

        // Find the starting node for the Eulerian path
        let mut start_node = pairs[0][0];
        for (&node, &out) in &out_degree {
            if out > *in_degree.get(&node).unwrap_or(&0) {
                start_node = node;
                break;
            }
        }

        // Hierholzer's algorithm to find Eulerian path
        let mut stack = Vec::new();
        let mut path = VecDeque::new();
        stack.push(start_node);

        while let Some(node) = stack.last() {
            if let Some(neighbors) = graph.get_mut(node) {
                if !neighbors.is_empty() {
                    let next_node = neighbors.pop().unwrap();
                    stack.push(next_node);
                } else {
                    path.push_front(stack.pop().unwrap());
                }
            } else {
                path.push_front(stack.pop().unwrap());
            }
        }

        // Convert the path into pairs
        let mut result = Vec::new();
        let path: Vec<i32> = path.into_iter().collect();
        for i in 0..path.len() - 1 {
            result.push(vec![path[i], path[i + 1]]);
        }
        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::valid_arrangement(
            [[5, 1], [4, 5], [11, 9], [9, 4]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // [[11,9],[9,4],[4,5],[5,1]]

    println!(
        "{:?}",
        Solution::valid_arrangement([[1, 3], [3, 2], [2, 1]].map(|e| e.to_vec()).to_vec())
    ); // [[1,3],[3,2],[2,1]]

    println!(
        "{:?}",
        Solution::valid_arrangement([[1, 2], [1, 3], [2, 1]].map(|e| e.to_vec()).to_vec())
    ); // [[1,2],[2,1],[1,3]]
}

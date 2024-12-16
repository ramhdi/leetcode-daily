// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/
// 2024/11/27

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..n - 1 {
            graph.entry(i).or_default().push(i + 1);
        }

        for query in queries {
            let (u, v) = (query[0], query[1]);
            graph.entry(u).or_default().push(v);

            let shortest_path = Self::bfs_shortest_path(&graph, 0, n - 1, n);
            result.push(shortest_path);
        }

        result
    }

    fn bfs_shortest_path(graph: &HashMap<i32, Vec<i32>>, start: i32, end: i32, n: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n as usize];
        queue.push_back((start, 0));
        visited[start as usize] = true;

        while let Some((node, dist)) = queue.pop_front() {
            if node == end {
                return dist;
            }

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !visited[neighbor as usize] {
                        visited[neighbor as usize] = true;
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }

        std::i32::MAX
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_distance_after_queries(
            5,
            [[2, 4], [0, 2], [0, 4]].map(|e| e.to_vec()).to_vec()
        )
    ); // [3,2,1]

    println!(
        "{:?}",
        Solution::shortest_distance_after_queries(4, [[0, 3], [0, 2]].map(|e| e.to_vec()).to_vec())
    ); // [1,1]
}

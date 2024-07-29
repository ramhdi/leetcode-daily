use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_groups(predators: Vec<i32>) -> i32 {
        let n = predators.len();
        // Create an adjacency list to represent the graph
        let mut graph = vec![Vec::new(); n];
        // Create an array to keep track of the indegree (number of incoming edges) of each node
        let mut indegree = vec![0; n];

        // Build the graph and calculate indegree of each node
        for (species, &predator) in predators.iter().enumerate() {
            if predator != -1 {
                graph[predator as usize].push(species);
                indegree[species] += 1;
            }
        }

        // Initialize a queue for topological sorting
        let mut queue = VecDeque::new();
        // Add all nodes with indegree 0 (nodes with no predators) to the queue
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        // BFS
        // Variable to keep track of the number of levels (groups)
        let mut level = 0;
        // Process nodes in topological order
        while !queue.is_empty() {
            // Increment the level for each new batch of nodes
            level += 1;
            // Process all nodes in the current level
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    // For each neighbor of the current node, decrement its indegree
                    for &neighbor in &graph[node] {
                        indegree[neighbor] -= 1;
                        // If a neighbor's indegree becomes 0, add it to the queue for the next level
                        if indegree[neighbor] == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        // The number of levels is the minimum number of groups needed
        level
    }
}

fn main() {
    let predators = vec![-1, 8, 6, 0, 7, 3, 8, 9, -1, 6, 1];
    let result = Solution::min_groups(predators);
    println!("{}", result); // Expected output: 5
}

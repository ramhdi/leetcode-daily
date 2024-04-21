// https://leetcode.com/problems/find-if-path-exists-in-graph/
// 2024/04/21

// use std::collections::{HashMap, VecDeque};

// Solution 1: matrix graph BFS --> Memory limit exceeded!
// impl Solution {
//     pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
//         let n = n as usize;
//         let mut graph_matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];

//         for edge in edges {
//             graph_matrix[edge[0] as usize][edge[1] as usize] = true;
//             graph_matrix[edge[1] as usize][edge[0] as usize] = true;
//         }

//         let mut queue: VecDeque<usize> = VecDeque::new();
//         let mut visited: Vec<bool> = vec![false; n];
//         queue.push_back(source as usize);
//         visited[source as usize] = true;

//         while let Some(current) = queue.pop_front() {
//             if current == destination as usize {
//                 return true;
//             }

//             for neighbor in 0..n {
//                 if graph_matrix[current][neighbor] && !visited[neighbor] {
//                     visited[neighbor] = true;
//                     queue.push_back(neighbor);
//                 }
//             }
//         }
//         return false;
//     }
// }

// Solution 2: hash table graph BFS --> OK!
// impl Solution {
//     pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
//         let n = n as usize;
//         let source = source as usize;
//         let destination = destination as usize;

//         let mut graph_map: HashMap<usize, Vec<usize>> = HashMap::new();

//         for edge in edges {
//             graph_map
//                 .entry(edge[0] as usize)
//                 .or_insert(Vec::new())
//                 .push(edge[1] as usize);

//             graph_map
//                 .entry(edge[1] as usize)
//                 .or_insert(Vec::new())
//                 .push(edge[0] as usize);
//         }

//         let mut queue: VecDeque<usize> = VecDeque::new();
//         let mut visited: Vec<bool> = vec![false; n];
//         queue.push_back(source as usize);
//         visited[source as usize] = true;

//         while let Some(current) = queue.pop_front() {
//             if current == destination as usize {
//                 return true;
//             }

//             for &neighbor in graph_map.get(&current).unwrap_or(&Vec::new()) {
//                 if !visited[neighbor] {
//                     visited[neighbor] = true;
//                     queue.push_back(neighbor);
//                 }
//             }
//         }

//         return false;
//     }
// }

// Solution 3: Union find --> most performant!
impl Solution {
    /// Function to determine if there is a valid path between the source and destination nodes.
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        // Initialize each node to be its own parent, setting up individual sets for each node.
        let mut parents: Vec<i32> = (0..n).collect();

        // Process all edges to unify nodes into connected components.
        for edge in edges {
            let node1 = edge[0];
            let node2 = edge[1];
            // Union the two nodes, linking their sets.
            Solution::union(node1, node2, &mut parents);
        }

        // Compare the roots of the source and destination nodes.
        // If they have the same root, they are connected.
        Solution::find(source, &mut parents) == Solution::find(destination, &mut parents)
    }

    /// Find the root of the node `x` with path compression.
    /// Path compression flattens the structure of the tree whenever `find` is called
    /// so that all nodes directly point to the root.
    pub fn find(x: i32, parents: &mut [i32]) -> i32 {
        // Check if we are at the root.
        if parents[x as usize] != x {
            // Path compression heuristic.
            // Recursively find the root and update the parent entry.
            parents[x as usize] = Solution::find(parents[x as usize], parents);
        }
        // Return the root.
        parents[x as usize]
    }

    /// Union the sets of nodes `x` and `y`.
    /// This function assumes `find` has been used to find the roots of `x` and `y`,
    /// and then links one root to the other to merge the sets.
    pub fn union(x: i32, y: i32, parents: &mut [i32]) {
        let root_x = Solution::find(x, parents);
        let root_y = Solution::find(y, parents);
        // Make one root the parent of the other, thus connecting the components.
        parents[root_y as usize] = root_x;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::valid_path(
            3,
            [[0, 1], [1, 2], [2, 0]].map(|r| r.to_vec()).to_vec(),
            0,
            2
        )
    ); // true

    println!(
        "{:?}",
        Solution::valid_path(
            6,
            [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
                .map(|r| r.to_vec())
                .to_vec(),
            0,
            5
        )
    ); // false
}

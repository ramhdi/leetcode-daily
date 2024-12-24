// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
// 2024/12/23

mod tree_node;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use tree_node::TreeNode;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let tree_levels = Self::bfs(root);
        let mut result = 0;

        for level in tree_levels {
            result += Self::min_swaps_to_sort(level);
        }

        result
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_values = Vec::new();

            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node_ref = node.borrow();
                    level_values.push(node_ref.val);

                    if let Some(left) = &node_ref.left {
                        queue.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &node_ref.right {
                        queue.push_back(Some(right.clone()));
                    }
                }
            }

            if !level_values.is_empty() {
                result.push(level_values);
            }
        }

        result
    }

    fn min_swaps_to_sort(arr: Vec<i32>) -> i32 {
        let mut arr_with_index: Vec<(usize, i32)> = arr.iter().cloned().enumerate().collect();
        arr_with_index.sort_by_key(|&(_, val)| val);

        let mut visited = vec![false; arr.len()];
        let mut swaps = 0;

        for i in 0..arr.len() {
            if visited[i] || arr_with_index[i].0 == i {
                continue;
            }

            let mut cycle_size = 0;
            let mut j = i;

            while !visited[j] {
                visited[j] = true;
                j = arr_with_index[j].0;
                cycle_size += 1;
            }

            if cycle_size > 1 {
                swaps += cycle_size - 1;
            }
        }

        swaps
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::minimum_operations(TreeNode::from_array(&[
            Some(1),
            Some(4),
            Some(3),
            Some(7),
            Some(6),
            Some(8),
            Some(5),
            None,
            None,
            None,
            None,
            Some(9),
            None,
            Some(10)
        ]))
    ); // 3

    println!(
        "{:?}",
        Solution::minimum_operations(TreeNode::from_array(
            &[1, 3, 2, 7, 6, 5, 4].map(|e| Some(e))
        ))
    ); // 3

    println!(
        "{:?}",
        Solution::minimum_operations(TreeNode::from_array(&[1, 2, 3, 4, 5, 6].map(|e| Some(e))))
    ); // 0
}

// https://leetcode.com/problems/even-odd-tree/
// 2024/02/29

mod tree_node;
use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut level = 0;

        while !queue.is_empty() {
            let mut level_values = Vec::new();
            let size = queue.len();

            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                level_values.push(node.val);

                if node.left.is_some() {
                    queue.push_back(node.left.clone().unwrap());
                }

                if node.right.is_some() {
                    queue.push_back(node.right.clone().unwrap());
                }
            }

            if !Self::check_level(&level_values, level) {
                return false;
            }

            level += 1;
        }

        true
    }

    fn check_level(values: &[i32], level: usize) -> bool {
        if level % 2 == 0 {
            for i in 0..values.len() {
                if values[i] % 2 == 0 {
                    return false;
                }
                if i > 0 && values[i] <= values[i - 1] {
                    return false;
                }
            }
        } else {
            for i in 0..values.len() {
                if values[i] % 2 != 0 {
                    return false;
                }
                if i > 0 && values[i] >= values[i - 1] {
                    return false;
                }
            }
        }

        true
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_even_odd_tree(TreeNode::from_array(
            &[
                Some(1),
                Some(10),
                Some(4),
                Some(3),
                None,
                Some(7),
                Some(9),
                Some(12),
                Some(8),
                Some(6),
                None,
                None,
                Some(2)
            ],
            0
        ))
    ); // true

    println!(
        "{:?}",
        Solution::is_even_odd_tree(TreeNode::from_array(
            &[Some(5), Some(4), Some(2), Some(3), Some(3), Some(7)],
            0
        ))
    ); // false

    println!(
        "{:?}",
        Solution::is_even_odd_tree(TreeNode::from_array(
            &[Some(5), Some(9), Some(1), Some(3), Some(5), Some(7)],
            0
        ))
    ); // false
}

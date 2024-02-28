// https://leetcode.com/problems/find-bottom-left-tree-value/
// 2024/02/28

mod tree_node;
use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = -1;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let length = queue.len();

            for i in 0..length {
                if let Some(node) = queue.pop_front() {
                    if i == 0 {
                        // the leftmost one
                        result = node.borrow().val;
                    }

                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_bottom_left_value(TreeNode::from_array(&[Some(2), Some(1), Some(3)], 0))
    ); // 1

    println!(
        "{:?}",
        Solution::find_bottom_left_value(TreeNode::from_array(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                Some(5),
                Some(6),
                None,
                None,
                Some(7)
            ],
            0
        ))
    ); // 7
}

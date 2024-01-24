// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
// 2024/01/24

mod tree_node;
use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = vec![];
        if let Some(node) = root {
            stack.push((node, 0));
        }

        let mut count = 0;

        while let Some((node, freq)) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            let freq = freq ^ (1 << node_ref.val);

            if node_ref.left.is_none() && node_ref.right.is_none() {
                count += ((freq & (freq - 1)) == 0) as i32;
                continue;
            }

            if let Some(right) = node_ref.right.take() {
                stack.push((right, freq));
            }
            if let Some(left) = node_ref.left.take() {
                stack.push((left, freq));
            }
        }

        return count;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::pseudo_palindromic_paths(TreeNode::from_array(
            &[Some(2), Some(3), Some(1), Some(3), Some(1), None, Some(1)],
            0,
        ))
    ); // 2

    println!(
        "{:?}",
        Solution::pseudo_palindromic_paths(TreeNode::from_array(
            &[
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(1),
            ],
            0,
        ))
    ); // 1
}

// https://leetcode.com/problems/diameter-of-binary-tree/
// 2024/02/27

mod tree_node;
use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Solution::dfs(&root, &mut max_diameter);

        max_diameter
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(inner) => {
                let inner = inner.borrow();
                let left_height = Solution::dfs(&inner.left, max_diameter);
                let right_height = Solution::dfs(&inner.right, max_diameter);
                *max_diameter = std::cmp::max(*max_diameter, left_height + right_height);

                std::cmp::max(left_height, right_height) + 1
            }
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::diameter_of_binary_tree(TreeNode::from_array(
            &[Some(1), Some(2), Some(3), Some(4), Some(5)],
            0
        ))
    ); // 3

    println!(
        "{:?}",
        Solution::diameter_of_binary_tree(TreeNode::from_array(&[Some(1), Some(2)], 0))
    ); // 1

    println!(
        "{:?}",
        Solution::diameter_of_binary_tree(TreeNode::from_array(&[Some(1)], 0))
    ); // 0
}

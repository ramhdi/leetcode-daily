// https://leetcode.com/problems/same-tree/
// 2024/02/26

mod tree_node;
use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();

                p.val == q.val
                    && Solution::is_same_tree(p.left.clone(), q.left.clone())
                    && Solution::is_same_tree(p.right.clone(), q.right.clone())
            }
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_same_tree(
            TreeNode::from_array(&[Some(1), Some(2), Some(3)],),
            TreeNode::from_array(&[Some(1), Some(2), Some(3)],)
        )
    ); // true

    println!(
        "{:?}",
        Solution::is_same_tree(
            TreeNode::from_array(&[Some(1), Some(2)],),
            TreeNode::from_array(&[Some(1), None, Some(2)],)
        )
    ); // false

    println!(
        "{:?}",
        Solution::is_same_tree(
            TreeNode::from_array(&[Some(1), Some(2), Some(1)],),
            TreeNode::from_array(&[Some(1), Some(1), Some(2)],)
        )
    ); // false
}

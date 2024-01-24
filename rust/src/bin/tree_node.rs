// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_array(arr: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if index >= arr.len() || arr[index].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(arr[index].unwrap())));
        let left_child = TreeNode::from_array(arr, 2 * index + 1);
        let right_child = TreeNode::from_array(arr, 2 * index + 2);

        root.borrow_mut().left = left_child;
        root.borrow_mut().right = right_child;

        return Some(root);
    }
}

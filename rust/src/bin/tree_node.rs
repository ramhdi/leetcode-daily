// Definition for a binary tree node.

use std::cell::RefCell;
use std::collections::VecDeque;
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

    pub fn to_array(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(Some(node));
        }

        while let Some(current) = queue.pop_front() {
            if let Some(node) = current {
                let node = node.borrow();
                result.push(Some(node.val));
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            } else {
                result.push(None);
            }
        }

        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }
}

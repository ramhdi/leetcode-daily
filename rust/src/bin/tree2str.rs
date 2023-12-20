// https://leetcode.com/problems/construct-string-from-binary-tree
// 2023/12/08

// Definition for a binary tree node.
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
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result: String = String::new();

        if let Some(root2) = root {
            let root3 = root2.borrow();
            let val = root3.val;
            let left = root3.left.clone();
            let right = root3.right.clone();

            result += &val.to_string();

            if left.is_some() || right.is_some() {
                result += &format!("({})", Solution::tree2str(left));
            }

            if right.is_some() {
                result += &format!("({})", Solution::tree2str(right));
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))))
    );
}

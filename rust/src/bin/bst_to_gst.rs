// https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
// 2024/06/25

mod tree_node;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn reverse_inorder(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                reverse_inorder(n.right.clone(), sum);
                *sum += n.val;
                n.val = *sum;
                reverse_inorder(n.left.clone(), sum);
            }
        }

        let mut sum = 0;
        reverse_inorder(root.clone(), &mut sum);
        root
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        TreeNode::to_array(Solution::bst_to_gst(TreeNode::from_array(&[
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8)
        ],)))
    ); // [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

    println!(
        "{:?}",
        TreeNode::to_array(Solution::bst_to_gst(TreeNode::from_array(&[
            Some(0),
            None,
            Some(1),
        ],)))
    ); // [1,null,1]
}

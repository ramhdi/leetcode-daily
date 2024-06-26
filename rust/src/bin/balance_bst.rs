// https://leetcode.com/problems/balance-a-binary-search-tree/
// 2024/06/26

mod tree_node;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn traverse_inorder(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                traverse_inorder(n.left.clone(), arr);
                arr.push(n.val);
                traverse_inorder(n.right.clone(), arr);
            }
        }

        fn construct_bst(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if arr.is_empty() {
                return None;
            }

            let mid = arr.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: arr[mid],
                left: construct_bst(&arr[0..mid]),
                right: construct_bst(&arr[mid + 1..]),
            })))
        }

        let mut sorted: Vec<i32> = Vec::new();
        traverse_inorder(root, &mut sorted);
        construct_bst(&sorted)
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        TreeNode::to_array(Solution::balance_bst(TreeNode::from_array(&[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ],)))
    ); // [2,1,3,null,null,null,4]

    println!(
        "{:?}",
        TreeNode::to_array(Solution::balance_bst(TreeNode::from_array(&[
            Some(2),
            Some(1),
            Some(3)
        ],)))
    ); // [2,1,3]
}

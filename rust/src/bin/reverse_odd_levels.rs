// https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
// 2024/12/20

mod tree_node;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node::TreeNode;
impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_rc) = root.clone() {
            let root_ref = root_rc.borrow();
            Self::dfs(&root_ref.left, &root_ref.right, 1);
        }

        root
    }

    fn dfs(
        left_node: &Option<Rc<RefCell<TreeNode>>>,
        right_node: &Option<Rc<RefCell<TreeNode>>>,
        level: i32,
    ) {
        match (left_node, right_node) {
            (None, None) => return,
            (Some(left), Some(right)) => {
                if level % 2 == 1 {
                    let (mut left_borrow, mut right_borrow) =
                        (left.borrow_mut(), right.borrow_mut());
                    std::mem::swap(&mut left_borrow.val, &mut right_borrow.val);
                }

                Self::dfs(&left.borrow().left, &right.borrow().right, level + 1);
                Self::dfs(&left.borrow().right, &right.borrow().left, level + 1);
            }
            _ => panic!("Invalid input! Need perfect tree"),
        }
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        TreeNode::to_array(Solution::reverse_odd_levels(TreeNode::from_array(
            &[2, 3, 5, 8, 13, 21, 34].map(|e| Some(e))
        )))
    ); // [2,5,3,8,13,21,34]

    println!(
        "{:?}",
        TreeNode::to_array(Solution::reverse_odd_levels(TreeNode::from_array(
            &[7, 13, 11].map(|e| Some(e))
        )))
    ); // [7,11,13]

    println!(
        "{:?}",
        TreeNode::to_array(Solution::reverse_odd_levels(TreeNode::from_array(
            &[0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].map(|e| Some(e))
        )))
    ); // [0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
}

// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// 2024/10/24

mod tree_node;
use std::cell::RefCell;
// use std::collections::HashMap;
use std::rc::Rc;
use tree_node::TreeNode;

// impl Solution {
//     pub fn flip_equiv(
//         root1: Option<Rc<RefCell<TreeNode>>>,
//         root2: Option<Rc<RefCell<TreeNode>>>,
//     ) -> bool {
//         let mut map1: HashMap<i32, (i32, i32)> = HashMap::new();
//         let mut map2: HashMap<i32, (i32, i32)> = HashMap::new();

//         Self::dfs(&root1, &mut map1);
//         Self::dfs(&root2, &mut map2);

//         println!("{:?}", map1);
//         println!("{:?}", map2);

//         if map1.len() != map2.len() {
//             return false;
//         }

//         for (root, (c1, c2)) in map1 {
//             match map2.get(&root) {
//                 Some(&(c3, c4)) => {
//                     if !((c1 == c3 && c2 == c4) || (c1 == c4 && c2 == c3)) {
//                         return false;
//                     }
//                 }
//                 None => return false,
//             }
//         }
//         true
//     }

//     fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut map: &mut HashMap<i32, (i32, i32)>) {
//         match node {
//             Some(node) => {
//                 let node = node.borrow();
//                 let left = &node.left;
//                 let right = &node.right;

//                 map.insert(
//                     node.val,
//                     (Self::get_node_val(left), Self::get_node_val(right)),
//                 );

//                 Self::dfs(left, &mut map);
//                 Self::dfs(right, &mut map);
//             }
//             None => (),
//         }
//     }

//     fn get_node_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match node {
//             Some(node) => node.borrow().val,
//             None => -1,
//         }
//     }
// }

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::dfs(&root1, &root2)
    }

    fn dfs(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => return true,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();

                if node1.val != node2.val {
                    return false;
                }

                let no_swap =
                    Self::dfs(&node1.left, &node2.left) && Self::dfs(&node1.right, &node2.right);

                let swap =
                    Self::dfs(&node1.left, &node2.right) && Self::dfs(&node1.right, &node2.left);

                swap || no_swap
            }
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::flip_equiv(
            TreeNode::from_array(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                None,
                None,
                None,
                Some(7),
                Some(8)
            ]),
            TreeNode::from_array(&[
                Some(1),
                Some(3),
                Some(2),
                None,
                Some(6),
                Some(4),
                Some(5),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(8),
                Some(7)
            ])
        )
    ); // true

    println!(
        "{:?}",
        Solution::flip_equiv(TreeNode::from_array(&[]), TreeNode::from_array(&[]))
    ); // true

    println!(
        "{:?}",
        Solution::flip_equiv(TreeNode::from_array(&[]), TreeNode::from_array(&[Some(1),]))
    ); // false
}

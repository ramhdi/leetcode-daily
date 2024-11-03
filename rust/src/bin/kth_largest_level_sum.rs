// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// 2024/10/22

mod tree_node;
use std::cell::RefCell;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;
use tree_node::TreeNode;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sum: BinaryHeap<i64> = BinaryHeap::new();

        // BFS
        if root.is_none() {
            return -1;
        }

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut sum: i64 = 0;
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val as i64;

                    if let Some(left) = &node.left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }

            level_sum.push(sum);
        }

        let mut i = 0;
        while let Some(sum) = level_sum.pop() {
            i += 1;
            if i == k {
                return sum;
            }
        }
        -1
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::kth_largest_level_sum(
            TreeNode::from_array(&[
                Some(5),
                Some(8),
                Some(9),
                Some(2),
                Some(1),
                Some(3),
                Some(7),
                Some(4),
                Some(6)
            ]),
            2
        )
    ); // 13

    println!(
        "{:?}",
        Solution::kth_largest_level_sum(
            TreeNode::from_array(&[Some(1), Some(2), None, Some(3)]),
            1
        )
    ); // 3
}

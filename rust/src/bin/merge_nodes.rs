// https://leetcode.com/problems/merge-nodes-in-between-zeros/
// 2024/07/04

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut sum = 0;
        let mut node = head;

        while let Some(mut boxed_node) = node {
            node = boxed_node.next.take();

            if boxed_node.val == 0 {
                if sum > 0 {
                    current.next = Some(Box::new(ListNode::new(sum)));
                    current = current.next.as_mut().unwrap();
                    sum = 0;
                }
            } else {
                sum += boxed_node.val;
            }
        }

        dummy.next
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::merge_nodes(ListNode::from_array(&[0, 3, 1, 0, 4, 5, 2, 0]))
    ); // [4,11]

    println!(
        "{:?}",
        Solution::merge_nodes(ListNode::from_array(&[0, 1, 0, 3, 0, 2, 2, 0]))
    ); // [1,3,4]
}

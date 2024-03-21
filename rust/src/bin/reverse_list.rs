// https://leetcode.com/problems/reverse-linked-list/
// 2024/03/21

mod list_node;
use crate::list_node::*;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut current) = head {
            head = current.next.take();
            current.next = prev;
            prev = Some(current);
        }

        return prev;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        ListNode::to_array(Solution::reverse_list(ListNode::from_array(&[
            1, 2, 3, 4, 5
        ])))
    ); // [5,4,3,2,1]

    println!(
        "{:?}",
        ListNode::to_array(Solution::reverse_list(ListNode::from_array(&[1, 2])))
    ); // [2,1]

    println!(
        "{:?}",
        ListNode::to_array(Solution::reverse_list(ListNode::from_array(&[])))
    ); // []
}

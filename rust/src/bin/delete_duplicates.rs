// https://leetcode.com/problems/remove-duplicates-from-sorted-list/

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = head.clone();
        let mut current = &mut result;

        while let Some(mut curr) = current.take() {
            while let Some(next) = curr.next.take() {
                if curr.val == next.val {
                    curr.next = next.next;
                } else {
                    curr.next = Some(next);
                    break;
                }
            }
            *current = Some(curr);
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::delete_duplicates(ListNode::from_array(&[1, 1, 2, 3, 3]))
    ); // [1,2,3]
}

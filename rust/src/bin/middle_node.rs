// https://leetcode.com/problems/middle-of-the-linked-list/
// 2024/03/07

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = head.clone();
        let mut i: usize = 0;

        while fast.is_some() {
            fast = fast.unwrap().next;
            i += 1;

            if i > 0 && i % 2 == 0 {
                slow = slow.unwrap().next;
            }
        }

        return slow;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::middle_node(ListNode::from_array(&[1, 2, 3, 4, 5]))
    ); // [3,4,5]

    println!(
        "{:?}",
        Solution::middle_node(ListNode::from_array(&[1, 2, 3, 4, 5, 6]))
    ); // [4,5,6]
}

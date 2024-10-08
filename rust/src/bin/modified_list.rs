// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array
// 2024/09/06

mod list_node;
use std::collections::HashSet;

use crate::list_node::ListNode;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums: HashSet<i32> = nums.into_iter().collect();
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut dummy;

        while let Some(next_node) = current.next.as_mut() {
            if nums.contains(&next_node.val) {
                current.next = next_node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        ListNode::to_array(Solution::modified_list(
            [1, 2, 3].to_vec(),
            ListNode::from_array(&[1, 2, 3, 4, 5])
        ))
    ); // [4,5]

    println!(
        "{:?}",
        ListNode::to_array(Solution::modified_list(
            [1].to_vec(),
            ListNode::from_array(&[1, 2, 1, 2, 1, 2])
        ))
    ); // [2,2,2]

    println!(
        "{:?}",
        ListNode::to_array(Solution::modified_list(
            [5].to_vec(),
            ListNode::from_array(&[1, 2, 3, 4])
        ))
    ); // [1,2,3,4]
}

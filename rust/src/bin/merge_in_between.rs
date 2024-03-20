// https://leetcode.com/problems/merge-in-between-linked-lists/
// 2024/03/20

mod list_node;
use crate::list_node::*;

use std::mem;

impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut list1;
        for _ in 0..a {
            current = &mut current.as_deref_mut().unwrap().next;
        }

        let mut list_1_rest = mem::replace(current, list2);
        for _ in a..=b {
            list_1_rest = list_1_rest.unwrap().next;
        }

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = list_1_rest;
        return list1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        ListNode::to_array(Solution::merge_in_between(
            ListNode::from_array(&[10, 1, 13, 6, 9, 5]),
            3,
            4,
            ListNode::from_array(&[1000000, 1000001, 1000002])
        ))
    ); // [10,1,13,1000000,1000001,1000002,5]

    println!(
        "{:?}",
        ListNode::to_array(Solution::merge_in_between(
            ListNode::from_array(&[0, 1, 2, 3, 4, 5, 6]),
            2,
            5,
            ListNode::from_array(&[1000000, 1000001, 1000002, 1000003, 1000004])
        ))
    ); // [0,1,1000000,1000001,1000002,1000003,1000004,6]
}

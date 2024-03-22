// https://leetcode.com/problems/reverse-linked-list/
// 2024/03/22

mod list_node;
use crate::list_node::*;

impl Solution {
    pub fn collect(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut node: Option<Box<ListNode>> = node;

        loop {
            match node {
                None => break,
                Some(value) => {
                    result.push(value.val);
                    node = value.next;
                }
            };
        }

        return result;
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let nodes: Vec<i32> = Solution::collect(head);
        for i in 0..nodes.len() {
            if nodes[i] != nodes[nodes.len() - i - 1] {
                return false;
            }
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_palindrome(ListNode::from_array(&[1, 2, 2, 1]))
    ); // true

    println!(
        "{:?}",
        Solution::is_palindrome(ListNode::from_array(&[1, 2,]))
    ); // false
}

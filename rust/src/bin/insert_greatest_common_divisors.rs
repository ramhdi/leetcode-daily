// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array
// 2024/09/06

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(ref mut node) = current {
            if node.next.is_none() {
                break;
            }

            let next_node = node.next.as_ref().unwrap();
            let gcd_val = Self::gcd(node.val, next_node.val);

            let new_node = Some(Box::new(ListNode {
                val: gcd_val,
                next: node.next.take(),
            }));

            node.next = new_node;

            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        ListNode::to_array(Solution::insert_greatest_common_divisors(
            ListNode::from_array(&[18, 6, 10, 3])
        ))
    ); // [18,6,6,2,10,1,3]

    println!(
        "{:?}",
        ListNode::to_array(Solution::insert_greatest_common_divisors(
            ListNode::from_array(&[7])
        ))
    ); // [7]
}

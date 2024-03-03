// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// 2024/03/03

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;

        let mut first = &mut dummy as *mut Option<Box<ListNode>>;
        let mut second = &mut dummy as *mut Option<Box<ListNode>>;

        unsafe {
            for _ in 0..=n {
                first = &mut (*first).as_mut().unwrap().next;
            }

            while !(*first).is_none() {
                first = &mut (*first).as_mut().unwrap().next;
                second = &mut (*second).as_mut().unwrap().next;
            }

            let tmp = &mut (*second).as_mut().unwrap().next;
            (*second).as_mut().unwrap().next = (*tmp).as_mut().unwrap().next.take();
        }

        dummy.unwrap().next
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::remove_nth_from_end(ListNode::from_array(&[1, 2, 3, 4, 5]), 2)
    ); // [1,2,3,5]

    println!(
        "{:?}",
        Solution::remove_nth_from_end(ListNode::from_array(&[1]), 1)
    ); // []

    println!(
        "{:?}",
        Solution::remove_nth_from_end(ListNode::from_array(&[1, 2]), 1)
    ); // [1]
}

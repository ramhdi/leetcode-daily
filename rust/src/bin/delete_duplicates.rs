// https://leetcode.com/problems/remove-duplicates-from-sorted-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
        Solution::delete_duplicates(Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode { next: None, val: 3 })),
                        val: 3
                    })),
                    val: 2
                })),
                val: 1
            })),
            val: 1
        })))
    );
}

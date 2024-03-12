// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// 2024/03/12

mod list_node;
use crate::list_node::ListNode;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Self::to_array(head);

        let mut has_zero_sum: bool = true;
        while has_zero_sum && result.len() >= 1 {
            let mut found_zero_sum: bool = false;
            let mut start: usize = 0;
            while !found_zero_sum && start < result.len() {
                let mut end: usize = start;
                while !found_zero_sum && end < result.len() {
                    let sum: i32 = result[start..=end].iter().sum();
                    if sum == 0 {
                        found_zero_sum = true;
                        let mut result_new: Vec<i32> = Vec::new();
                        for i in 0..result.len() {
                            if i < start || i > end {
                                result_new.push(result[i]);
                            }
                        }
                        result = result_new;
                    }
                    end += 1;
                }
                start += 1;
            }

            if !found_zero_sum {
                has_zero_sum = false;
            }
        }

        return Self::from_array(&result);
    }

    fn from_array(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &num in nums.iter() {
            *current = Some(Box::new(ListNode::new(num)));
            current = &mut current.as_mut().unwrap().next;
        }

        return head;
    }

    fn to_array(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let mut curr = head;
        while curr.is_some() {
            let curr_content = curr.unwrap();
            result.push(curr_content.val);
            curr = curr_content.next;
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[1, 2, -3, 3, 1]))
    ); // [3,1]

    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[1, 2, 3, -3, 4]))
    ); // [1,2,4]

    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[1, 2, 3, -3, -2]))
    ); // [1]

    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[0]))
    ); // []

    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[0, 0]))
    ); // []

    println!(
        "{:?}",
        Solution::remove_zero_sum_sublists(ListNode::from_array(&[2, 0]))
    ); // [2]
}

// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array
// 2024/09/06

mod list_node;

use crate::list_node::ListNode;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize];
        let mut current = &head;
        let (mut i, mut j): (i32, i32) = (0, 0);
        let dir: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut curr_dir: usize = 0;

        while let Some(node) = current {
            result[i as usize][j as usize] = node.val;

            let (mut ni, mut nj) = (i + dir[curr_dir].0, j + dir[curr_dir].1);
            if ni < 0 || nj < 0 || ni >= m || nj >= n || result[ni as usize][nj as usize] != -1 {
                curr_dir = (curr_dir + 1) % 4;
                (ni, nj) = (i + dir[curr_dir].0, j + dir[curr_dir].1);
            }

            (i, j) = (ni, nj);

            current = &node.next;
        }

        result
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::spiral_matrix(
            3,
            5,
            ListNode::from_array(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])
        )
    ); // [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]

    println!(
        "{:?}",
        Solution::spiral_matrix(1, 4, ListNode::from_array(&[0, 1, 2]))
    ); // [[0,1,2,-1]]
}

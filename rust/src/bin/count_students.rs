// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
// 2024/04/08

use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = VecDeque::from(students);
        let mut sandwiches = VecDeque::from(sandwiches);
        let mut left_count = students.len();
        let mut counter = 0;

        while left_count != 0 && counter < left_count {
            if students[0] == sandwiches[0] {
                students.pop_front();
                sandwiches.pop_front();
                left_count -= 1;
                counter = 0;
            } else {
                let student = students.pop_front().unwrap();
                students.push_back(student);
                counter += 1;
            }
        }

        left_count as _
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_students([1, 1, 0, 0].to_vec(), [0, 1, 0, 1].to_vec())
    ); // 0

    println!(
        "{:?}",
        Solution::count_students([1, 1, 1, 0, 0, 1].to_vec(), [1, 0, 0, 0, 1, 1].to_vec())
    ); // 3
}

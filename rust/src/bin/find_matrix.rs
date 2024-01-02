// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
// 2024/01/02

use std::collections::HashSet;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        let mut freq: Vec<usize> = vec![0; nums.len() + 1];
        for num in nums {
            if freq[num as usize] >= result.len() {
                result.push(vec![]);
            }
            result[freq[num as usize]].push(num);
            freq[num as usize] += 1;
        }
        return result;
    }

    pub fn _find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        let mut counter: Vec<HashSet<i32>> = vec![HashSet::new()];
        for num in nums {
            let mut i: usize = 0;
            while i < counter.len() && counter[i].contains(&num) {
                i += 1;
            }
            if i == counter.len() {
                result.push(vec![]);
                counter.push(HashSet::new());
            }
            result[i].push(num);
            counter[i].insert(num);
        }
        return result;
    }
}
pub struct Solution {}

fn main() {
    println!("{:?}", Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]));
    println!("{:?}", Solution::find_matrix(vec![1, 3, 4, 2]));
}

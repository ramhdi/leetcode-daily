// https://leetcode.com/problems/combination-sum/

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current_combination = Vec::new();

        Self::backtrack(
            &candidates,
            target,
            0,
            &mut current_combination,
            &mut result,
        );

        return result;
    }

    fn backtrack(
        candidates: &Vec<i32>,
        remaining_target: i32,
        start_index: usize,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining_target == 0 {
            result.push(current_combination.clone());
            return;
        }

        for i in start_index..candidates.len() {
            let candidate = candidates[i];
            if candidate <= remaining_target {
                current_combination.push(candidate);
                Self::backtrack(
                    candidates,
                    remaining_target - candidate,
                    i,
                    current_combination,
                    result,
                );
                current_combination.pop();
            }
        }
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
    println!("{:?}", Solution::combination_sum(vec![2, 3, 5], 8));
    println!("{:?}", Solution::combination_sum(vec![2], 1));
}

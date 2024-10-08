// https://leetcode.com/problems/combination-sum-ii/
// 2024/08/13

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current_combination: Vec<i32> = Vec::new();

        Self::backtrack(
            &candidates,
            target,
            0,
            &mut current_combination,
            &mut result,
        );

        result
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current_combination.clone());
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            if candidates[i] > target {
                break;
            }

            current_combination.push(candidates[i]);

            Self::backtrack(
                candidates,
                target - candidates[i],
                i + 1,
                current_combination,
                result,
            );

            current_combination.pop();
        }
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::combination_sum2([10, 1, 2, 7, 6, 1, 5].to_vec(), 8)
    ); // [[1,1,6],[1,2,5],[1,7],[2,6]]

    println!(
        "{:?}",
        Solution::combination_sum2([2, 5, 2, 1, 2].to_vec(), 5)
    ); // [[1,2,2],[5]]
}

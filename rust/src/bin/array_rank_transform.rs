// https://leetcode.com/problems/rank-transform-of-an-array/
// 2024/10/02

use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let sorted = {
            let mut temp = arr.clone();
            temp.sort_unstable();
            temp
        };

        let mut rank: HashMap<i32, i32> = HashMap::new();
        let mut curr_rank = 0;
        let mut curr_num = std::i32::MIN;

        for num in sorted {
            if num == curr_num {
                continue;
            }

            curr_num = num;
            curr_rank += 1;
            rank.insert(curr_num, curr_rank);
        }

        arr.iter()
            .map(|num| rank.get(num).unwrap().to_owned())
            .collect()
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::array_rank_transform([40, 10, 20, 30].to_vec())
    ); // [4,1,2,3]

    println!(
        "{:?}",
        Solution::array_rank_transform([100, 100, 100].to_vec())
    ); // [1,1,1]

    println!(
        "{:?}",
        Solution::array_rank_transform([37, 12, 28, 9, 100, 56, 80, 5, 12].to_vec())
    ); // [5,3,4,2,8,6,7,1,3]
}

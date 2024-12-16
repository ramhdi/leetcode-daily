// https://leetcode.com/problems/special-array-ii/
// 2024/12/09

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let group = nums.windows(2).fold(vec![0], |mut acc, pair| {
            let last_group = *acc.last().unwrap();
            if (pair[0] ^ pair[1]) & 1 == 0 {
                acc.push(last_group + 1);
            } else {
                acc.push(last_group);
            }
            acc
        });

        queries
            .iter()
            .map(|query| {
                let (start, end) = (query[0] as usize, query[1] as usize);
                group[start] == group[end]
            })
            .collect()
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_array_special(
            [3, 4, 1, 2, 6].to_vec(),
            [[0, 4]].map(|e| e.to_vec()).to_vec()
        )
    ); // [false]

    println!(
        "{:?}",
        Solution::is_array_special(
            [4, 3, 1, 6].to_vec(),
            [[0, 2], [2, 3]].map(|e| e.to_vec()).to_vec()
        )
    ); // [false,true]
}

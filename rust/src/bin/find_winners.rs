// https://leetcode.com/problems/find-players-with-zero-or-one-losses/
// 2024/01/15

use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners: Vec<i32> = vec![];
        let mut lose_ones: Vec<i32> = vec![];

        let mut lose_count: HashMap<i32, u128::MAX> = HashMap::new();

        for m in matches {
            *lose_count.entry(m[0]).or_insert(0) += 0;
            *lose_count.entry(m[1]).or_insert(0) += 1;
        }

        for (k, v) in lose_count {
            if v == 0 {
                winners.push(k);
            }

            if v == 1 {
                lose_ones.push(k)
            }
        }

        winners.sort_unstable();
        lose_ones.sort_unstable();

        return vec![winners, lose_ones];
    }

    pub fn _find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut l = HashMap::new();
        let mut w = HashMap::new();

        for i in matches {
            *w.entry(i[0]).or_insert(0) += 1;
            *l.entry(i[1]).or_insert(0) += 1;
        }

        let mut res1 = w
            .keys()
            .filter(|&x| !l.contains_key(x))
            .map(|x| *x)
            .collect::<Vec<_>>();

        let mut res2 = l
            .into_iter()
            .filter(|x| x.1 <= 1)
            .map(|x| x.0)
            .collect::<Vec<i32>>();

        res1.sort_unstable();
        res2.sort_unstable();

        vec![res1, res2]
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_winners(
            [
                [1, 3],
                [2, 3],
                [3, 6],
                [5, 6],
                [5, 7],
                [4, 5],
                [4, 8],
                [4, 9],
                [10, 4],
                [10, 9]
            ]
            .map(|e| e.to_vec())
            .to_vec()
        )
    ); // [[1,2,10],[4,5,7,8]]
    println!(
        "{:?}",
        Solution::find_winners(
            [[2, 3], [1, 3], [5, 4], [6, 4]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // [[1,2,5,6],[]]
}

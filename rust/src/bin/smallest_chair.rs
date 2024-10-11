// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/
// 2024/10/11

use std::collections::HashMap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut friends: Vec<(i32, i32, i32)> = times
            .iter()
            .enumerate()
            .map(|(f, t)| (f as i32, t[0], t[1]))
            .collect();

        friends.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", friends);

        let mut chairs: HashMap<i32, i32> = HashMap::new();
        let mut max_chair = -1;
        for (friend, arrival, leaving) in friends {
            let mut seated = false;
            let mut selected_seat = -1;
            for c in 0..=max_chair {
                if chairs.get(&c).is_none() || *chairs.get(&c).unwrap() <= arrival {
                    chairs.insert(c, leaving);
                    seated = true;
                    selected_seat = c;
                    break;
                }
            }

            if !seated {
                max_chair += 1;
                chairs.insert(max_chair, leaving);
                selected_seat = max_chair;
            }

            if friend == target_friend {
                return selected_seat;
            }
        }

        std::i32::MAX
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::smallest_chair([[1, 4], [2, 3], [4, 6]].map(|e| e.to_vec()).to_vec(), 1)
    ); // 1

    println!(
        "{:?}",
        Solution::smallest_chair([[3, 10], [1, 5], [2, 6]].map(|e| e.to_vec()).to_vec(), 0)
    ); // 2

    println!(
        "{:?}",
        Solution::smallest_chair(
            [
                [33889, 98676],
                [80071, 89737],
                [44118, 52565],
                [52992, 84310],
                [78492, 88209],
                [21695, 67063],
                [84622, 95452],
                [98048, 98856],
                [98411, 99433],
                [55333, 56548],
                [65375, 88566],
                [55011, 62821],
                [48548, 48656],
                [87396, 94825],
                [55273, 81868],
                [75629, 91467]
            ]
            .map(|e| e.to_vec())
            .to_vec(),
            6
        )
    ); // 2
}

// https://leetcode.com/problems/hand-of-straights/
// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
// 2024/06/06

use std::borrow::BorrowMut;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }

        hand.sort_unstable();
        let mut card_groups: Vec<(i32, usize)> = Vec::with_capacity(hand.len() / group_size);

        for card in hand {
            let mut found_group: bool = false;
            let mut i: usize = 0;
            while !found_group && i < card_groups.len() {
                if card_groups[i].1 < group_size && card_groups[i].0 == card - 1 {
                    let group = card_groups[i].borrow_mut();
                    group.0 = card;
                    group.1 += 1;
                    found_group = true;
                }
                i += 1;
            }

            if !found_group {
                card_groups.push((card, 1));
            }
        }

        println!("{:?}", card_groups);

        for (_, group_len) in card_groups {
            if group_len != group_size {
                return false;
            }
        }

        return true;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::is_n_straight_hand([1, 2, 3, 6, 2, 3, 4, 7, 8].to_vec(), 3)
    ); // true

    println!(
        "{:?}",
        Solution::is_n_straight_hand([1, 2, 3, 4, 5].to_vec(), 4)
    ); // false

    println!(
        "{:?}",
        Solution::is_n_straight_hand([1, 2, 3, 3, 4, 4, 5, 6].to_vec(), 4)
    ); // true

    println!(
        "{:?}",
        Solution::is_n_straight_hand([3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11].to_vec(), 3)
    ); // true

    println!(
        "{:?}",
        Solution::is_n_straight_hand([1, 2, 3, 4].to_vec(), 3)
    ); // false
}

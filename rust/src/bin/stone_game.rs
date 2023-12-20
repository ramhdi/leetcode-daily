// https://leetcode.com/problems/stone-game/

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        // return true;
        let piles_sorted: Vec<i32> = {
            let mut temp = piles;
            temp.sort();
            temp.reverse();
            temp
        };

        let mut sum_even: i32 = 0;
        let mut sum_odd: i32 = 0;

        for (i, &p) in piles_sorted.iter().enumerate() {
            if i % 2 == 0 {
                sum_even += p;
            } else {
                sum_odd += p;
            }
        }
        return sum_even > sum_odd;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::stone_game(vec![5, 3, 4, 5]));
}

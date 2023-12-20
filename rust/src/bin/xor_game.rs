// https://leetcode.com/problems/chalkboard-xor-game/

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        return nums.len() % 2 == 0 || nums.into_iter().reduce(|x, y| (x ^ y)).unwrap() == 0;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::xor_game(vec![1, 1, 2]));
}

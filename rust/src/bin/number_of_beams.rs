// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
// 2024/01/03

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut result: i32 = 0;
        let mut prev_laser_count: i32 = 0;
        for b in bank {
            let laser_count = b.chars().filter(|c| c == &'1').count() as i32;
            if laser_count > 0 {
                result += laser_count * prev_laser_count;
                prev_laser_count = laser_count;
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::number_of_beams(
            ["011001", "000000", "010100", "001000"]
                .map(|s| s.to_string())
                .to_vec()
        )
    );
    println!(
        "{:?}",
        Solution::number_of_beams(["000", "111", "000"].map(|s| s.to_string()).to_vec())
    );
}

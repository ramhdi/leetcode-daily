// https://leetcode.com/contest/weekly-contest-402/problems/count-pairs-that-form-a-complete-day-i/
// 2024/06/16

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        if hours.len() < 2 {
            return 0;
        }

        let mut result: i32 = 0;

        for i in 1..hours.len() {
            for j in 0..i {
                if (hours[i] + hours[j]) % 24 == 0 {
                    result += 1;
                }
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_complete_day_pairs([12, 12, 30, 24, 24].to_vec())
    ); // 2

    println!(
        "{:?}",
        Solution::count_complete_day_pairs([72, 48, 24, 3].to_vec())
    ); // 3
}

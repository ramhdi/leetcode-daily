// https://leetcode.com/problems/time-needed-to-buy-tickets/
// 2024/04/09

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut result: i32 = 0;
        let k = k as usize;

        while tickets[k] > 0 {
            for i in 0..tickets.len() {
                if tickets[i] > 0 {
                    result += 1;
                    tickets[i] -= 1;
                }

                if i == k && tickets[k] == 0 {
                    return result;
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
        Solution::time_required_to_buy([2, 3, 2].to_vec(), 2)
    ); // 6

    println!(
        "{:?}",
        Solution::time_required_to_buy([5, 1, 1, 1].to_vec(), 0)
    ); // 8
}

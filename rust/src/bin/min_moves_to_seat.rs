// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/
// 2024/06/13

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        seats
            .iter()
            .zip(students.iter())
            .fold(0, |res, (seat, student)| res + (seat - student).abs())
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_moves_to_seat([3, 1, 5].to_vec(), [2, 7, 4].to_vec())
    ); // 4

    println!(
        "{:?}",
        Solution::min_moves_to_seat([4, 1, 5, 9].to_vec(), [1, 3, 2, 6].to_vec())
    ); // 7

    println!(
        "{:?}",
        Solution::min_moves_to_seat([2, 2, 6, 6].to_vec(), [1, 3, 2, 6].to_vec())
    ); // 4
}

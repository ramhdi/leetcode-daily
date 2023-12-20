// https://leetcode.com/problems/minimum-time-to-complete-trips/

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut i: i64 = 1;
        let mut trips: i32 = 0;
        while trips < total_trips {
            for t in time.clone() {
                trips += (i as i32 % t == 0) as i32;
            }
            i += 1;
        }

        return i - 1;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::minimum_time(vec![1, 2, 3], 5));
    println!("{:?}", Solution::minimum_time(vec![2], 1));
    println!("{:?}", Solution::minimum_time(vec![5, 10, 10], 9));
}

// https://leetcode.com/problems/climbing-stairs/
// 2024/01/18

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        let mut dp: (i32, i32, i32) = (2, 1, 1);
        for _ in 2..=n {
            dp.0 = dp.1 + dp.2;
            dp.2 = dp.1;
            dp.1 = dp.0;
        }
        return dp.0;
    }
}

pub struct Solution {}

fn main() {
    // println!("{:?}", Solution::climb_stairs(2)); // 2
    // println!("{:?}", Solution::climb_stairs(3)); // 3
    for i in 0..=45 {
        println!("climb_stairs({i}) = {:?}", Solution::climb_stairs(i));
    }
}

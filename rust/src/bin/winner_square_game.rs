// https://leetcode.com/problems/stone-game-iv/

impl Solution {
    pub fn nearest_square(n: i32) -> i32 {
        let root: i32 = (n as f64).sqrt() as i32;
        return root * root;
    }

    pub fn is_square(n: i32) -> bool {
        return n == Solution::nearest_square(n);
    }

    pub fn winner_square_game(n: i32) -> bool {
        if Solution::is_square(n) {
            return true;
        }
        if Solution::is_square(n - Solution::nearest_square(n)) {
            return false;
        }
        return Solution::winner_square_game(
            n - Solution::nearest_square(n)
                - Solution::nearest_square(n - Solution::nearest_square(n)),
        );
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::winner_square_game(4));
}

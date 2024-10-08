// https://leetcode.com/problems/lemonade-change/
// 2024/08/15

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0, 0);

        for b in bills {
            match b {
                5 => fives += 1,
                10 => {
                    if fives == 0 {
                        return false;
                    }

                    fives -= 1;
                    tens += 1;
                }
                20 => {
                    if tens == 0 && fives >= 3 {
                        fives -= 3;
                    } else if tens >= 1 && fives >= 1 {
                        fives -= 1;
                        tens -= 1;
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }

        true
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::lemonade_change([5, 5, 5, 10, 20].to_vec())); // true
    println!("{}", Solution::lemonade_change([5, 5, 10, 10, 20].to_vec())); // false
}

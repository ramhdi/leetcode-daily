// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
// 2024/10/09

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (add_open, add_close) =
            s.chars()
                .fold((0i32, 0i32), |(mut add_open, mut add_close), c| {
                    match c {
                        '(' => add_close += 1,
                        ')' => {
                            if add_close > 0 {
                                add_close -= 1;
                            } else {
                                add_open += 1;
                            }
                        }
                        _ => (),
                    }

                    (add_open, add_close)
                });

        add_open + add_close
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_add_to_make_valid("())".to_string())); // 1

    println!("{:?}", Solution::min_add_to_make_valid("(((".to_string())); // 3

    println!(
        "{:?}",
        Solution::min_add_to_make_valid("()))((".to_string())
    ); // 4
}

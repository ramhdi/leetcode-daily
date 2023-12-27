// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
// 2023/12/27

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut temp_char: Option<char> = None;
        let mut result = 0;
        let mut same_color_time_total = 0;
        let mut same_color_time_max = 0;

        for (i, color) in colors.chars().enumerate() {
            let char = color;

            if i == 0 {
                temp_char = Some(char);
                same_color_time_total += needed_time[i];
                if needed_time[i] > same_color_time_max {
                    same_color_time_max = needed_time[i];
                }
            } else {
                if temp_char == Some(char) {
                    same_color_time_total += needed_time[i];
                    if needed_time[i] > same_color_time_max {
                        same_color_time_max = needed_time[i];
                    }
                    if i == colors.len() - 1 {
                        result = result + same_color_time_total - same_color_time_max;
                    }
                } else {
                    temp_char = Some(char);
                    result = result + same_color_time_total - same_color_time_max;
                    same_color_time_total = needed_time[i];
                    same_color_time_max = needed_time[i];
                }
            }
        }

        return result;
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
    );
    println!("{:?}", Solution::min_cost("abc".to_string(), vec![1, 2, 3]));
    println!(
        "{:?}",
        Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1])
    );
}

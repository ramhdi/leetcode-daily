// https://leetcode.com/problems/count-the-number-of-fair-pairs/
// 2024/11/13

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let n = nums.len();
        let mut result: i64 = 0;

        for i in 0..n - 1 {
            let low = i
                + 1
                + match nums[i + 1..].binary_search_by(|&x| {
                    if x + nums[i] >= lower {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }) {
                    Ok(pos) => pos,
                    Err(pos) => pos,
                };

            let high = i
                + 1
                + match nums[i + 1..].binary_search_by(|&x| {
                    if x + nums[i] > upper {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }) {
                    Ok(pos) => pos + 1,
                    Err(pos) => pos,
                };

            if low < high {
                result += (high - low) as i64;
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_fair_pairs([0, 1, 7, 4, 4, 5].to_vec(), 3, 6)
    ); // 6

    println!(
        "{:?}",
        Solution::count_fair_pairs([1, 7, 9, 2, 5].to_vec(), 11, 11)
    ); // 1

    println!(
        "{:?}",
        Solution::count_fair_pairs([5, 7, 5, 7, 5].to_vec(), 12, 12)
    ); // 6

    println!(
        "{:?}",
        Solution::count_fair_pairs([0, 0, 0, 0, 0, 0].to_vec(), 0, 0)
    ); // 15
}

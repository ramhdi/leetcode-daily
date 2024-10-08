// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/
// 2024/09/02

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let total_chalk: i64 = chalk.iter().map(|&c| c as i64).sum();
        let mut k = (k as i64) % total_chalk;

        for (i, &c) in chalk.iter().enumerate() {
            if k < c as i64 {
                return i as i32;
            }
            k -= c as i64;
        }

        0
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::chalk_replacer([5, 1, 5].to_vec(), 22)); // 0

    println!("{}", Solution::chalk_replacer([3, 4, 1, 2].to_vec(), 25)); // 1

    println!("{}", Solution::chalk_replacer([1].to_vec(), 1000000000)); // 0
}

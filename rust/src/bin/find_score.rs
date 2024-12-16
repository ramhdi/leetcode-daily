// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/
// 2024/12/13

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_unstable_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        nums.into_iter()
            .fold((vec![false; n], 0), |mut acc, (i, num)| {
                if !acc.0[i] {
                    acc.1 += num as i64;
                    if i > 0 {
                        acc.0[i - 1] = true;
                    }
                    if i < n - 1 {
                        acc.0[i + 1] = true;
                    }
                }

                acc
            })
            .1
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::find_score([2, 1, 3, 4, 5, 2].to_vec())); // 7

    println!("{:?}", Solution::find_score([2, 3, 5, 1, 3, 2].to_vec())); // 5
}

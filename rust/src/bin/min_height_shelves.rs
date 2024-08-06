// https://leetcode.com/problems/filling-bookcase-shelves/
// 2024/07/31

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp: Vec<i32> = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let (mut curr_w, mut curr_h): (i32, i32) = (0, 0);
            for j in (1..=i).rev() {
                curr_w += books[j - 1][0];
                if curr_w <= shelf_width {
                    curr_h = std::cmp::max(curr_h, books[j - 1][1]);
                    dp[i] = std::cmp::min(dp[i], dp[j - 1] + curr_h);
                }
            }
        }

        return dp[n];
    }
}
struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::min_height_shelves(
            [[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]]
                .map(|p| p.to_vec())
                .to_vec(),
            4
        )
    ); // 6

    println!(
        "{}",
        Solution::min_height_shelves([[1, 3], [2, 4], [3, 2]].map(|p| p.to_vec()).to_vec(), 6)
    ); // 4
}

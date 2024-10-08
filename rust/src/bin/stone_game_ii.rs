// https://leetcode.com/problems/stone-game-ii/
// 2024/08/20

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut suffix_sum = vec![0; n + 1];

        // Compute the suffix sums
        for i in (0..n).rev() {
            suffix_sum[i] = piles[i] + suffix_sum[i + 1];
        }

        let mut memo = vec![vec![-1; n]; n];

        // Start the recursion from pile 0 and M = 1
        return Self::dp(0, 1, &piles, &suffix_sum, &mut memo);
    }

    fn dp(
        i: usize,
        m: usize,
        piles: &Vec<i32>,
        suffix_sum: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let n = piles.len();

        // Base case: If we have reached the end, no more stones can be taken
        if i >= n {
            return 0;
        }

        // If all remaining stones can be taken, return the sum of the remaining stones
        if i + 2 * m >= n {
            return suffix_sum[i];
        }

        // If we already computed this state, return the cached value
        if memo[i][m] != -1 {
            return memo[i][m];
        }

        let mut best = 0;

        // Try taking `x` piles (1 <= x <= 2M) and maximize the result
        for x in 1..=2 * m {
            best = best.max(suffix_sum[i] - Self::dp(i + x, m.max(x), piles, suffix_sum, memo));
        }

        // Memoize the result
        memo[i][m] = best;

        best
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::stone_game_ii(vec![2, 7, 9, 4, 4])); // 10
    println!("{}", Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100])); // 104
}

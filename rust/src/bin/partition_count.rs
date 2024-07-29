struct Solution;

impl Solution {
    fn partition_count(n: usize, k: usize) -> usize {
        // Create a DP array with size (n + 1) initialized to zero
        let mut dp = vec![0; n + 1];

        // There is one way to partition 0: using zero parts
        dp[0] = 1;

        // Fill the DP array
        for j in 1..=k {
            for i in j..=n {
                dp[i] = dp[i] + dp[i - j];
            }
        }

        dp[n]
    }
}

fn main() {
    let n = 8;
    let k = 3;
    let result = Solution::partition_count(n, k);
    println!("{}", result); // Expected output: 10
}

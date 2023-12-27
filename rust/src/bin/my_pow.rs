// https://leetcode.com/problems/powx-n/

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n < 0 {
            x = 1.0 / x;
        }

        let mut result: f64 = 1.0;
        let mut num: i64 = (n as i64).abs();
        while num != 0 {
            if num & 1 != 0 {
                result *= x;
            }
            x *= x;
            num >>= 1;
        }

        return result;
    }

    pub fn _my_pow(x: f64, n: i32) -> f64 {
        let mut dp: Vec<f64> = vec![-1.0; n.abs() as usize + 1];

        fn dp_helper(x: f64, n: i32, dp: &mut Vec<f64>) -> f64 {
            if n < 0 {
                return 1.0 / dp_helper(x, -n, dp);
            }
            if dp[n as usize] != -1.0 {
                return dp[n as usize];
            }
            if x == 0.0 {
                let result = 0.0;
                dp[n as usize] = result;
                return result;
            }
            if n > 0 {
                let result = x * dp_helper(x, n - 1, dp);
                dp[n as usize] = result;
                return result;
            }
            let result = 1.0;
            dp[n as usize] = result;
            return result;
        }

        for i in 0..n {
            dp_helper(x, i, &mut dp);
        }
        return dp_helper(x, n, &mut dp);
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::my_pow(2.00000, 10));
    println!("{}", Solution::my_pow(2.00000, 3));
    println!("{}", Solution::my_pow(2.00000, -2));
    println!("{}", Solution::my_pow(1.0000000000001, -2147483648));
}

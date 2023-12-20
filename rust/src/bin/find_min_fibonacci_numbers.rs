// https://leetcode.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/

impl Solution {
    pub fn generate_fibonacci(k: i32) -> Vec<i32> {
        if k == 1 {
            return vec![1, 1];
        }

        let mut result: Vec<i32> = vec![1, 1];
        let mut prev: i32 = 1;
        let mut second_prev: i32 = 1;
        let mut curr: i32 = prev + second_prev;
        while curr <= k {
            result.push(curr);
            second_prev = prev;
            prev = curr;
            curr = prev + second_prev;
        }
        return result;
    }

    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut result: i32 = 1;
        let mut fib_seq: Vec<i32> = Solution::generate_fibonacci(k);
        let mut diff: i32 = k - fib_seq.pop().unwrap();
        while diff > 0 {
            while fib_seq.last().unwrap() > &diff {
                fib_seq.pop();
            }
            diff = diff - fib_seq.pop().unwrap();
            result += 1;
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::find_min_fibonacci_numbers(7));
    println!("{}", Solution::find_min_fibonacci_numbers(10));
    println!("{}", Solution::find_min_fibonacci_numbers(19));
    println!("{}", Solution::find_min_fibonacci_numbers(1));
    println!("{}", Solution::find_min_fibonacci_numbers(2));
}

// https://leetcode.com/problems/daily-temperatures/
// 2024/01/31

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if temperatures[i] > temperatures[top] {
                    result[top] = (i - top) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73].to_vec())
    ); // [1,1,4,2,1,1,0,0]

    println!(
        "{:?}",
        Solution::daily_temperatures([30, 40, 50, 60].to_vec())
    ); // [1,1,1,0]

    println!("{:?}", Solution::daily_temperatures([30, 60, 90].to_vec())); // [1,1,0]
}

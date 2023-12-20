// https://leetcode.com/problems/largest-number/

impl Solution {
    fn custom_sort(a: &str, b: &str) -> std::cmp::Ordering {
        let numeric_a = a.parse::<i32>().unwrap_or(i32::MAX);
        let numeric_b = b.parse::<i32>().unwrap_or(i32::MAX);

        match (numeric_a, numeric_b) {
            (i32::MAX, i32::MAX) => a.cmp(b),
            (i32::MAX, _) => std::cmp::Ordering::Greater,
            (_, i32::MAX) => std::cmp::Ordering::Less,
            (x, y) if x == y => a.cmp(b),
            (x, y) => x.cmp(&y),
        }
    }

    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str: Vec<String> = nums.into_iter().map(|n| n.to_string()).collect();
        nums_str.sort_by(|a, b| Solution::custom_sort(a, b));
        return nums_str
            .into_iter()
            .reduce(|acc, n| acc + &n)
            .unwrap_or(String::new());
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::largest_number(vec![3, 30, 34, 5, 9]));
}

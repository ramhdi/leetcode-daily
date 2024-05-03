// https://leetcode.com/problems/compare-version-numbers/
// 2024/05/03

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = Self::split_and_convert(&version1);
        let v2 = Self::split_and_convert(&version2);

        println!("v1={:?} v2={:?}", v1, v2);
        let n1 = v1.len();
        let n2 = v2.len();
        let n = std::cmp::max(n1, n2);
        let mut last1: i32;
        let mut last2: i32;

        for i in 0..n {
            if i < n1 {
                last1 = v1[i];
            } else {
                last1 = 0;
            }

            if i < n2 {
                last2 = v2[i];
            } else {
                last2 = 0;
            }

            if last1 < last2 {
                return -1;
            }

            if last1 > last2 {
                return 1;
            }
        }

        return 0;
    }

    fn split_and_convert(input: &str) -> Vec<i32> {
        input
            .split('.')
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect()
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::compare_version("1.01".to_string(), "1.001".to_string(),)
    ); // 0

    println!(
        "{:?}",
        Solution::compare_version("1.0".to_string(), "1.0.0".to_string(),)
    ); // 0

    println!(
        "{:?}",
        Solution::compare_version("0.1".to_string(), "1.1".to_string(),)
    ); // -1

    println!(
        "{:?}",
        Solution::compare_version("1.0.1".to_string(), "1".to_string(),)
    ); // 1
}

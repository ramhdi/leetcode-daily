// https://leetcode.com/problems/maximum-distance-in-arrays/
// 2024/08/16

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut min1, mut max1) = (std::i32::MAX, std::i32::MIN);
        let (mut min2, mut max2) = (std::i32::MAX, std::i32::MIN);

        for arr in arrays {
            let n = arr.len();
            min = min.min(arr[0]);
            max = max.max(arr[n - 1]);
        }

        max - min
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]])
    ); // 4

    println!("{}", Solution::max_distance(vec![vec![1], vec![1]])); // 0
}

// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
// 2024/11/14

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;

            if Self::can_distribute(&mid, &quantities, &n) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_distribute(x: &i32, quantities: &[i32], n: &i32) -> bool {
        (quantities
            .iter()
            .map(|&q| (q as u32).div_ceil(*x as u32))
            .sum::<u32>() as i32)
            <= *n
    }
}
struct Solution {}

fn main() {
    println!("{:?}", Solution::minimized_maximum(6, [11, 6].to_vec())); // 3

    println!(
        "{:?}",
        Solution::minimized_maximum(7, [15, 10, 10].to_vec())
    ); // 5

    println!("{:?}", Solution::minimized_maximum(1, [100000].to_vec())); // 100000
}

// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
// 2024/11/20

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }

        let b = s.as_bytes();
        let n = b.len();
        let total_count: [i32; 3] = b.iter().fold([0; 3], |mut acc, x| {
            acc[(x - b'a') as usize] += 1;
            acc
        });

        if total_count.iter().any(|&c| c < k) {
            return -1;
        }

        let mut window_count: [i32; 3] = [0; 3];
        let mut left = 0;
        let mut max_window_size = 0;

        for right in 0..n {
            window_count[(b[right] - b'a') as usize] += 1;

            while (0..3).any(|i| total_count[i] - window_count[i] < k) {
                window_count[(b[left] - b'a') as usize] -= 1;
                left += 1;
            }

            max_window_size = max_window_size.max((right as i32 - left as i32 + 1) as usize);
        }

        (n - max_window_size) as i32
    }
}
struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::take_characters("aabaaaacaabc".to_string(), 2)
    ); // 8

    println!("{:?}", Solution::take_characters("a".to_string(), 1)); // -1

    println!("{:?}", Solution::take_characters("aabbccca".to_string(), 2)); // 6
}

struct Solution;

impl Solution {
    pub fn slowest_key(key_times: &[Vec<i32>]) -> char {
        let mut max_duration = 0;
        let mut slowest_key = ' ';

        for i in 0..key_times.len() {
            let duration = if i == 0 {
                key_times[i][1]
            } else {
                key_times[i][1] - key_times[i - 1][1]
            };

            let key = (b'a' + key_times[i][0] as u8) as char;

            if duration > max_duration || (duration == max_duration && key < slowest_key) {
                max_duration = duration;
                slowest_key = key;
            }
        }

        slowest_key
    }
}

fn main() {
    let key_times = vec![vec![0, 2], vec![1, 5], vec![0, 9], vec![2, 15]];
    let result = Solution::slowest_key(&key_times);
    println!("{}", result); // Expected output: 'c'
}

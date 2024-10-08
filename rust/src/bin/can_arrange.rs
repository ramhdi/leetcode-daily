// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
// 2024/10/01

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }

        let mut freq: Vec<i32> = vec![0; k as usize];
        for x in arr {
            freq[(((x % k) + k) % k) as usize] += 1;
        }
        println!("{:?}", freq);

        if freq[0] % 2 != 0 {
            return false;
        }

        for i in 1..=(k / 2) {
            if freq[i as usize] != freq[(k - i) as usize] {
                return false;
            }
        }

        true
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::can_arrange([1, 2, 3, 4, 5, 10, 6, 7, 8, 9].to_vec(), 5)
    ); // true

    println!(
        "{:?}",
        Solution::can_arrange([1, 2, 3, 4, 5, 6].to_vec(), 7)
    ); // true

    println!(
        "{:?}",
        Solution::can_arrange([1, 2, 3, 4, 5, 6].to_vec(), 10)
    ); // false
}

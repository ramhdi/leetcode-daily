// https://leetcode.com/problems/move-pieces-to-obtain-a-string/
// 2024/12/05

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (start, target) = (start.as_bytes(), target.as_bytes());
        let mut i = 0;
        let mut j = 0;
        let n = start.len();

        while i < n || j < n {
            while i < n && start[i] == b'_' {
                i += 1;
            }
            while j < n && target[j] == b'_' {
                j += 1;
            }

            if (i < n) != (j < n) {
                return false;
            }

            if i >= n && j >= n {
                break;
            }

            if start[i] != target[j] {
                return false;
            }

            if start[i] == b'L' && i < j {
                return false;
            }
            if start[i] == b'R' && i > j {
                return false;
            }

            i += 1;
            j += 1;
        }

        true
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string())
    ); // true

    println!(
        "{:?}",
        Solution::can_change("R_L_".to_string(), "__LR".to_string())
    ); // false

    println!(
        "{:?}",
        Solution::can_change("_R".to_string(), "R_".to_string())
    ); // false

    println!(
        "{:?}",
        Solution::can_change("_L__R__R_L".to_string(), "L______RR_".to_string())
    ); // false
}

// https://leetcode.com/problems/find-champion-ii/
// 2024/11/26

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut in_degrees = vec![0; n as usize];
        for edge in edges {
            in_degrees[edge[1] as usize] += 1;
        }

        let mut zeros = (0, 0);
        for (n, &in_degree) in in_degrees.iter().enumerate() {
            if in_degree == 0 {
                zeros.0 += 1;
                zeros.1 = n;
            }
        }

        if zeros.0 == 1 {
            zeros.1 as i32
        } else {
            -1
        }
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_champion(3, [[0, 1], [1, 2]].map(|e| e.to_vec()).to_vec())
    ); // 0

    println!(
        "{:?}",
        Solution::find_champion(4, [[0, 2], [1, 3], [1, 2]].map(|e| e.to_vec()).to_vec())
    ); // -1
}

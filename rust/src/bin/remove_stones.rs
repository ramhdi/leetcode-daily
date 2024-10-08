// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
// 2024/08/30

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut groups = 0usize;
        let mut visited = vec![false; n];

        fn dfs(index: usize, stones: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
            visited[index] = true;
            let (x1, y1) = (stones[index][0], stones[index][1]);

            for i in 0..stones.len() {
                if !visited[i] {
                    let (x2, y2) = (stones[i][0], stones[i][1]);
                    if x1 == x2 || y1 == y2 {
                        dfs(i, stones, visited);
                    }
                }
            }
        }

        for i in 0..n {
            if !visited[i] {
                dfs(i, &stones, &mut visited);
                groups += 1;
            }
        }

        (n - groups) as i32
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::remove_stones(
            [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 5

    println!(
        "{}",
        Solution::remove_stones(
            [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 3
}

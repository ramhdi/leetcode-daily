// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/
// 2024/11/28

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dist = vec![vec![usize::MAX; n]; m];
        dist[0][0] = 0;

        let mut deque = VecDeque::new();
        deque.push_back((0usize, 0usize));

        let directions = [(0isize, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((x, y)) = deque.pop_front() {
            for &(dx, dy) in &directions {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let cost = grid[ny][nx] as usize;
                    if dist[y][x] + cost < dist[ny][nx] {
                        dist[ny][nx] = dist[y][x] + cost;

                        if cost == 0 {
                            deque.push_front((nx, ny));
                        } else {
                            deque.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        dist[m - 1][n - 1] as i32
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::minimum_obstacles(
            [[0, 1, 1], [1, 1, 0], [1, 1, 0]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 2

    println!(
        "{:?}",
        Solution::minimum_obstacles(
            [[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 0
}

// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/
// 2024/11/29

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct State {
    time: i32,
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut min_time = vec![vec![i32::MAX; n]; m];
        min_time[0][0] = 0;

        let mut pq = BinaryHeap::new();
        pq.push(State {
            time: 0,
            row: 0,
            col: 0,
        });

        let directions = [(0isize, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(State { time, row, col }) = pq.pop() {
            if time > min_time[row][col] {
                continue;
            }

            if row == m - 1 && col == n - 1 {
                return time;
            }

            for &(dx, dy) in &directions {
                let (nx, ny) = (col as isize + dx, row as isize + dy);
                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let mut next_time = time + 1;
                    if next_time < grid[ny][nx] {
                        next_time = grid[ny][nx];
                        if (next_time - time) % 2 == 0 {
                            next_time += 1;
                        }
                    }

                    if next_time < min_time[ny][nx] {
                        min_time[ny][nx] = next_time;
                        pq.push(State {
                            time: next_time,
                            row: ny,
                            col: nx,
                        });
                    }
                }
            }
        }

        -1
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::minimum_time(
            [[0, 1, 3, 2], [5, 1, 2, 5], [4, 3, 8, 6]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 7

    println!(
        "{:?}",
        Solution::minimum_time(
            [[0, 2, 4], [3, 2, 1], [1, 0, 4]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // -1
}

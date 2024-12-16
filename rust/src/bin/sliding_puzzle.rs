// https://leetcode.com/problems/sliding-puzzle/
// 2024/11/25

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target = "123450".to_string();
        let start = Self::board_to_string(&board);

        if start == target {
            return 0;
        }

        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        queue.push_back((start.clone(), 0));
        visited.insert(start);

        while let Some((curr_state, moves)) = queue.pop_front() {
            let curr_board: Vec<Vec<i32>> = curr_state
                .chars()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|&c| c.to_digit(10).unwrap() as i32)
                        .collect()
                })
                .collect();

            for next_board in Self::get_neighbors(&curr_board) {
                let next_state = Self::board_to_string(&next_board);
                if next_state == target {
                    return moves + 1;
                }
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, moves + 1));
                }
            }
        }

        -1
    }

    fn board_to_string(board: &[Vec<i32>]) -> String {
        board
            .iter()
            .flat_map(|row| row.iter())
            .map(|&x| x.to_string())
            .collect()
    }

    fn get_neighbors(board: &[Vec<i32>]) -> Vec<Vec<Vec<i32>>> {
        let mut zero_pos = (0, 0);
        for i in 0..2 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    zero_pos = (i, j);
                    break;
                }
            }
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbors = Vec::new();

        for (dx, dy) in directions.iter() {
            let new_x = zero_pos.0 as i32 + dx;
            let new_y = zero_pos.1 as i32 + dy;
            if new_x >= 0 && new_x < 2 && new_y >= 0 && new_y < 3 {
                let mut new_board = board.to_vec();
                new_board[zero_pos.0][zero_pos.1] = new_board[new_x as usize][new_y as usize];
                new_board[new_x as usize][new_y as usize] = 0;
                neighbors.push(new_board);
            }
        }
        neighbors
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::sliding_puzzle([[1, 2, 3], [4, 0, 5]].map(|e| e.to_vec()).to_vec())
    ); // 1

    println!(
        "{:?}",
        Solution::sliding_puzzle([[1, 2, 3], [5, 4, 0]].map(|e| e.to_vec()).to_vec())
    ); // -1

    println!(
        "{:?}",
        Solution::sliding_puzzle([[4, 1, 2], [5, 0, 3]].map(|e| e.to_vec()).to_vec())
    ); // -1
}

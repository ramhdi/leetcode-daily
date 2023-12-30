// https://leetcode.com/problems/word-search/

use std::collections::HashSet;

impl Solution {
    fn find_neighbor(
        (i, j): (usize, usize),
        previous: HashSet<(usize, usize)>,
        target_char: char,
        board: Vec<Vec<char>>,
    ) -> Vec<(usize, usize)> {
        let m = board.len();
        let n = board[0].len();
        let mut result: Vec<(usize, usize)> = Vec::new();
        if i >= 1 && !previous.contains(&(i - 1, j)) && board[i - 1][j] == target_char {
            result.push((i - 1, j));
        }
        if i < m - 1 && !previous.contains(&(i + 1, j)) && board[i + 1][j] == target_char {
            result.push((i + 1, j));
        }
        if j >= 1 && !previous.contains(&(i, j - 1)) && board[i][j - 1] == target_char {
            result.push((i, j - 1));
        }
        if j < n - 1 && !previous.contains(&(i, j + 1)) && board[i][j + 1] == target_char {
            result.push((i, j + 1));
        }
        return result;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let mut curr_points: Vec<(usize, usize)> = Vec::new();
        let mut chars: HashSet<char> = HashSet::new();
        for (i, row) in board.clone().into_iter().enumerate() {
            for (j, ch) in row.into_iter().enumerate() {
                chars.insert(ch);
                if ch == word[0] {
                    curr_points.push((i, j));
                }
            }
        }

        if curr_points.len() == 0 {
            return false;
        }

        for ch in word.clone() {
            if !chars.contains(&ch) {
                return false;
            }
        }

        let mut curr_tracks: Vec<HashSet<(usize, usize)>> = Vec::new();
        for p in curr_points.clone() {
            let mut track: HashSet<(usize, usize)> = HashSet::new();
            track.insert(p);
            curr_tracks.push(track);
        }

        for k in 1..word.len() {
            let mut next_points: Vec<(usize, usize)> = Vec::new();
            let mut next_tracks: Vec<HashSet<(usize, usize)>> = Vec::new();
            for (l, p) in curr_points.into_iter().enumerate() {
                let mut next_point =
                    Solution::find_neighbor(p, curr_tracks[l].clone(), word[k], board.clone());
                let mut next_track: Vec<HashSet<(usize, usize)>> = Vec::new();
                for p in next_point.clone() {
                    let mut curr_track = curr_tracks[l].clone();
                    curr_track.insert(p);
                    next_track.push(curr_track);
                }
                next_points.append(&mut next_point);
                next_tracks.append(&mut next_track);
            }
            if next_points.len() == 0 {
                return false;
            }
            curr_points = next_points;
            curr_tracks = next_tracks;
        }
        if curr_points.len() > 0 {
            return true;
        }
        return false;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
            ],
            "AAAAAAAAAAAABAA".to_string()
        )
    );
}

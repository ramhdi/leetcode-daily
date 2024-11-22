// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// 2024/11/21

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut cells = vec![vec![(false, false); n as usize]; m as usize]; // (occupied, visible)

        // Occupy cells
        for c in guards.iter().chain(walls.iter()) {
            cells[c[0] as usize][c[1] as usize].0 = true;
        }

        // Mark cells visible by guards
        let mut visible_cells = 0;
        for g in guards.iter() {
            // Go on each cardinal direction
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .for_each(|&(dx, dy)| {
                    let (mut cx, mut cy) = (g[1], g[0]);
                    while cx + dx >= 0
                        && cx + dx < n
                        && cy + dy >= 0
                        && cy + dy < m
                        && !cells[(cy + dy) as usize][(cx + dx) as usize].0
                    {
                        cx += dx;
                        cy += dy;
                        if !cells[cy as usize][cx as usize].1 {
                            visible_cells += 1;
                        }
                        cells[cy as usize][cx as usize].1 = true;
                    }
                });
        }

        // Count unguarded cells
        (m * n) - visible_cells - guards.len() as i32 - walls.len() as i32
    }
}
struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::count_unguarded(
            4,
            6,
            [[0, 0], [1, 1], [2, 3]].map(|e| e.to_vec()).to_vec(),
            [[0, 1], [2, 2], [1, 4]].map(|e| e.to_vec()).to_vec()
        )
    ); // 7

    println!(
        "{:?}",
        Solution::count_unguarded(
            3,
            3,
            [[1, 1]].map(|e| e.to_vec()).to_vec(),
            [[0, 1], [1, 0], [2, 1], [1, 2]]
                .map(|e| e.to_vec())
                .to_vec()
        )
    ); // 4
}

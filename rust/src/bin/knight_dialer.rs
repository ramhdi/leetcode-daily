// https://leetcode.com/problems/knight-dialer
// 2023/11/27

// fn knight_moves(n: i32) -> Vec<i32> {
//     match n {
//         1 => return vec![6, 8],
//         2 => return vec![7, 9],
//         3 => return vec![4, 8],
//         4 => return vec![0, 3, 9],
//         5 => return vec![],
//         6 => return vec![0, 1, 7],
//         7 => return vec![2, 6],
//         8 => return vec![1, 3],
//         9 => return vec![2, 4],
//         0 => return vec![4, 6],
//         _ => return vec![],
//     }
// }

// fn knight_moves_n(start: i32, length: i32) -> u64 {
//     match length {
//         1 => return 1,
//         _ => {
//             let mut result: u64 = 0;
//             let moves = knight_moves(start);
//             for &m in moves.iter() {
//                 result += knight_moves_n(m, length - 1) as u64;
//             }
//             return result;
//         }
//     }
// }

// fn knight_dialer(n: i32) -> i32 {
//     let mut result: u64 = 0;

//     for i in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
//         result += knight_moves_n(i, n);
//     }
//     return (result % 1_000_000_007) as i32;
// }

fn knight_dialer(n: i32) -> i32 {
    if n == 1 {
        return 10;
    }

    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (4, 2, 2, 1);
    const MOD: i32 = 1_000_000_007;

    for _ in 0..(n - 1) {
        let temp_a = a;
        let temp_b = b;
        let temp_c = c;
        let temp_d = d;

        a = ((2 * temp_b) % MOD + (2 * temp_c) % MOD) % MOD;
        b = temp_a;
        c = (temp_a + (2 * temp_d) % MOD) % MOD;
        d = temp_c;
    }

    let mut ans = (a + b) % MOD;
    ans = (ans + c) % MOD;
    return (ans + d) % MOD;
}

fn main() {
    println!("{}", knight_dialer(3131));
}

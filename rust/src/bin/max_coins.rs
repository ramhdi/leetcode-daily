// https://leetcode.com/problems/maximum-number-of-coins-you-can-get
// 2023/11/24

fn max_coins(piles: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    let piles_sorted = {
        let mut sorted = piles.clone();
        sorted.sort();
        sorted
    };

    let mut i = piles_sorted.len() / 3 as usize;
    while i < (piles_sorted.len() as usize) {
        result += piles_sorted[i];
        i += 2;
    }

    return result;
}

fn main() {
    println!("{}", max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
}

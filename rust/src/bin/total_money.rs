// https://leetcode.com/problems/calculate-money-in-leetcode-bank
// 2023/12/06

fn total_money(n: i32) -> i32 {
    let n2 = n / 7;
    let n3 = n % 7;
    return n2 * 28 + 7 * ((n2 - 1) * (n2)) / 2 + (n3 * (n3 + 1)) / 2 + n3 * n2;
}

fn main() {
    println!("{}", total_money(20));
}

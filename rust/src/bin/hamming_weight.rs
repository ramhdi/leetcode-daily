// https://leetcode.com/problems/number-of-1-bits
// 2023/11/29

fn hamming_weight(n: u32) -> i32 {
    let mut result: i32 = 0;
    let mut n2 = n;
    while n2 > 0 {
        result += (n2 % 2) as i32;
        n2 >>= 1;
    }
    return result;
}

fn main() {
    println!("{}", hamming_weight(0b11111111111111111111111111111101));
}

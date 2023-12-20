// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero
// 2023/11/30

// fn is_power_of_two(n: i32) -> bool {
//     return n != 0 && (n & (n - 1)) == 0;
// }

// fn to_xor(n: i32) -> i32 {
//     if n % 2 == 1 {
//         return 1;
//     } else if is_power_of_two(n) {
//         return n;
//     } else {
//         let mut div = 2;
//         let mut result = 0;

//         while result == 0 {
//             result = n % div;
//             div *= 2;
//         }

//         return result;
//     }
// }

// fn minimum_one_bit_operations(n: i32) -> i32 {
//     if n == 0 {
//         return 0;
//     }

//     if is_power_of_two(n) {
//         return 2 * n - 1;
//     }

//     let mut result: i32 = 1;
//     let mut start = 0;
//     while start != n {
//         start ^= to_xor(result);
//         result += 1;
//     }

//     return result - 1;
// }

fn minimum_one_bit_operations(n: i32) -> i32 {
    let mut ans = n;
    ans ^= ans >> 16;
    ans ^= ans >> 8;
    ans ^= ans >> 4;
    ans ^= ans >> 2;
    ans ^= ans >> 1;
    return ans;
}

fn main() {
    println!("{}", minimum_one_bit_operations(131656520));
}

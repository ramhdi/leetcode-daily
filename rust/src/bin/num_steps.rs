// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
// 2024/05/29

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let (operations, carry) =
            s.bytes()
                .skip(1)
                .rev()
                .fold((0, 0), |(mut result, mut carry), b| {
                    let current = (b - b'0') as i32 + carry;
                    if current % 2 == 1 {
                        result += 2;
                        carry = 1;
                    } else {
                        result += 1;
                    }
                    (result, carry)
                });

        operations + carry
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::num_steps("1101".to_string())); // 6

    println!("{:?}", Solution::num_steps("10".to_string())); // 1

    println!("{:?}", Solution::num_steps("1".to_string())); // 0
}

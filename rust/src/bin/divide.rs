// https://leetcode.com/problems/divide-two-integers/

impl Solution {
    pub fn divide(mut dividend: i32, divisor: i32) -> i32 {
        let mut result: i32 = 0;
        if dividend == 0 {
            return 0;
        } else if divisor == 1 {
            return dividend as i32;
        } else if divisor == -1 {
            if dividend > i32::MIN {
                return 0 - dividend;
            } else {
                return i32::MAX;
            }
        } else if dividend > 0 {
            if divisor > 0 {
                while dividend >= divisor {
                    dividend -= divisor;
                    if result < i32::MAX {
                        result += 1;
                    }
                }
            } else {
                while dividend >= -divisor {
                    dividend += divisor;
                    if result > i32::MIN {
                        result -= 1;
                    }
                }
            }
        } else {
            if divisor > 0 {
                while dividend <= -divisor {
                    dividend += divisor;
                    if result > i32::MIN {
                        result -= 1;
                    }
                }
            } else {
                while dividend <= divisor {
                    dividend -= divisor;
                    if result < i32::MAX {
                        result += 1;
                    }
                }
            }
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::divide(1, 1));
    println!("{:?}", Solution::divide(10, 3));
    println!("{:?}", Solution::divide(10, -3));
    println!("{:?}", Solution::divide(-10, 3));
    println!("{:?}", Solution::divide(-10, -3));
    println!("{:?}", Solution::divide(i32::MAX, 1));
    println!("{:?}", Solution::divide(i32::MIN, -3));
}

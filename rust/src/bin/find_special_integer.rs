// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array
// 2023/12/11

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut count = 0;
        let len = arr.len();

        for n in arr {
            if n != result {
                result = n;
                count = 1;
            } else {
                count += 1;
                if count > len / 4 {
                    return result;
                }
            }
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::find_special_integer(vec![1, 1]));
}

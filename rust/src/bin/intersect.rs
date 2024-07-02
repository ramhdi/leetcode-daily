// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// 2024/07/02

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut count1: [usize; 1001] = [0; 1001];
        let mut count2: [usize; 1001] = [0; 1001];

        for n1 in nums1 {
            count1[n1 as usize] += 1;
        }

        for n2 in nums2 {
            count2[n2 as usize] += 1;
        }

        let mut result: Vec<i32> = Vec::new();

        for n in 0..1001 {
            for _ in 0..std::cmp::min(count1[n as usize], count2[n as usize]) {
                result.push(n);
            }
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::intersect([1, 2, 2, 1].to_vec(), [2, 2].to_vec())
    ); // [2,2]

    println!(
        "{:?}",
        Solution::intersect([4, 9, 5].to_vec(), [9, 4, 9, 8, 4].to_vec())
    ); // [4,9]
}

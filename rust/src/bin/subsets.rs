// https://leetcode.com/problems/subsets/
// 2024/05/21

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << n);

        for i in 0..(1 << n) {
            let mut subset: Vec<i32> = Vec::new();
            for j in 0..n {
                if i & (1 << j) != 0 {
                    subset.push(nums[j]);
                }
            }

            result.push(subset);
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::subsets([1, 2, 3].to_vec())); // [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

    println!("{:?}", Solution::subsets([0].to_vec())); // [[],[0]]
}

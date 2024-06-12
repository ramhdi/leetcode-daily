// https://leetcode.com/problems/sort-colors/
// 2024/06/12

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count: [u16; 3] = [0; 3];

        for num in nums.into_iter() {
            count[*num as usize] += 1;
        }

        let mut last_index: usize = 0;
        for i in 0..3 {
            for _ in 0..count[i] {
                nums[last_index] = i as i32;
                last_index += 1;
            }
        }
    }
}

pub struct Solution {}

fn main() {
    let mut test1 = [2, 0, 2, 1, 1, 0].to_vec();
    Solution::sort_colors(&mut test1);
    println!("{:?}", test1); // [0,0,1,1,2,2]

    let mut test2 = [2, 0, 1].to_vec();
    Solution::sort_colors(&mut test2);
    println!("{:?}", test2); // [0,1,2]
}

// https://leetcode.com/problems/find-if-array-can-be-sorted/
// 2024/11/06

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut segments: Vec<(i32, i32)> = Vec::new();
        let (mut seg_min, mut seg_max, mut last_num) = (nums[0], nums[0], nums[0]);
        for n in nums {
            if n.count_ones() == last_num.count_ones() {
                seg_min = std::cmp::min(seg_min, n);
                seg_max = std::cmp::max(seg_max, n);
            } else {
                segments.push((seg_min, seg_max));
                seg_min = n;
                seg_max = n;
            }
            last_num = n;
        }
        segments.push((seg_min, seg_max));
        println!("{:?}", segments);

        segments.windows(2).all(|p| p[0].1 < p[1].0)
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::can_sort_array([8, 4, 2, 30, 15].to_vec())); // true

    println!("{:?}", Solution::can_sort_array([1, 2, 3, 4, 5].to_vec())); // true

    println!("{:?}", Solution::can_sort_array([3, 16, 8, 4, 2].to_vec())); // true
}

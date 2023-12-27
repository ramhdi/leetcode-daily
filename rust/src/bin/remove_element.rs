// https://leetcode.com/problems/remove-element/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[index] = nums[i];
                index += 1;
            }
        }
        println!("{:?}", nums);
        return index as i32;
    }

    pub fn _remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count: i32 = 0;
        let len = nums.len() as i32;
        for (i, &num) in nums.clone().iter().enumerate() {
            if num == val {
                count += 1;
                nums[i] = i32::MAX;
            }
        }
        nums.sort();
        println!("{:?}", nums);
        return len - count;
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::remove_element(vec![3, 2, 2, 3].as_mut(), 3)
    );
    println!(
        "{:?}",
        Solution::remove_element(vec![0, 1, 2, 2, 3, 0, 4, 2].as_mut(), 2)
    );
}

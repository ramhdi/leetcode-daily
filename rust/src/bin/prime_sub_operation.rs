// https://leetcode.com/problems/prime-subtraction-operation/description/
// 2024/11/11

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let lpb = Self::largest_prime_before(1000);
        let mut prev_num = -1;

        for i in 0..nums.len() {
            if i == 0 {
                if nums[i] >= 2 {
                    nums[i] -= lpb[nums[i] as usize];
                }
            } else {
                if nums[i] <= prev_num {
                    return false;
                } else {
                    let delta = nums[i] - prev_num;
                    nums[i] -= lpb[delta as usize];
                }
            }

            prev_num = nums[i];
        }

        true
    }

    fn largest_prime_before(limit: usize) -> Vec<i32> {
        let mut is_prime = vec![true; limit + 1];
        let mut largest_prime = vec![0; limit + 1];

        is_prime[0] = false;
        if limit > 0 {
            is_prime[1] = false;
        }

        let mut last_prime = 0;
        for num in 2..=limit {
            if is_prime[num] {
                largest_prime[num] = last_prime;
                last_prime = num as i32;

                let mut multiple = num * 2;
                while multiple <= limit {
                    is_prime[multiple] = false;
                    multiple += num;
                }
            } else {
                largest_prime[num] = last_prime;
            }
        }

        largest_prime
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::prime_sub_operation([4, 9, 6, 10].to_vec())); // true

    println!("{}", Solution::prime_sub_operation([6, 8, 11, 12].to_vec())); // true

    println!("{}", Solution::prime_sub_operation([5, 8, 3].to_vec())); // false

    println!("{}", Solution::prime_sub_operation([2, 2].to_vec())); // false
}

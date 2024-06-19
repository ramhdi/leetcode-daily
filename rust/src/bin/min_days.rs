// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
// 2024/06/19

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let n = bloom_day.len();
        if n < (m * k) as usize {
            return -1;
        }

        let (mut min_day, mut max_day): (i32, i32) = (i32::MAX, 0);
        for day in bloom_day.clone() {
            min_day = min_day.min(day);
            max_day = max_day.max(day);
        }

        while min_day <= max_day {
            let day = min_day + (max_day - min_day) / 2;
            if Self::can_make_bouquets(day, m, k, &bloom_day) {
                max_day = day - 1;
            } else {
                min_day = day + 1;
            }
        }

        return min_day;
    }

    fn can_make_bouquets(current_day: i32, m: i32, k: i32, bloom_day: &Vec<i32>) -> bool {
        let mut consecutive_bloom: i32 = 0;
        let mut made_bouquets: i32 = 0;

        for &day in bloom_day {
            if day <= current_day {
                consecutive_bloom += 1;

                if consecutive_bloom == k {
                    made_bouquets += 1;
                    consecutive_bloom = 0;
                }
            } else {
                consecutive_bloom = 0;
            }

            if made_bouquets >= m {
                return true;
            }
        }

        return false;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_days([1, 10, 3, 10, 2].to_vec(), 3, 1)); // 3

    println!("{:?}", Solution::min_days([1, 10, 3, 10, 2].to_vec(), 3, 2)); // -1

    println!(
        "{:?}",
        Solution::min_days([7, 7, 7, 7, 12, 7, 7].to_vec(), 2, 3)
    ); // 12
}

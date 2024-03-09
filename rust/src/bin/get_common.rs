// https://leetcode.com/problems/minimum-common-value/
// 2024/03/09

// impl Solution {
//     pub fn get_common(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
//         let mut nums1 = nums1.into_iter();
//         let mut nums2 = nums2.into_iter();
//         let mut n1 = nums1.next();
//         let mut n2 = nums2.next();
//         loop {
//             match (n1, n2) {
//                 (Some(a), Some(b)) => {
//                     if a < b {
//                         n1 = nums1.next();
//                     } else if b < a {
//                         n2 = nums2.next();
//                     } else if a == b {
//                         return a;
//                     }
//                 }
//                 _ => return -1,
//             }
//         }
//     }
// }

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j): (usize, usize) = (0, 0);

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                return nums1[i];
            }
        }

        return -1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::get_common([1, 2, 3].to_vec(), [2, 4].to_vec())
    ); // 2

    println!(
        "{:?}",
        Solution::get_common([1, 2, 3, 6].to_vec(), [2, 3, 4, 5].to_vec())
    ); // 2
}

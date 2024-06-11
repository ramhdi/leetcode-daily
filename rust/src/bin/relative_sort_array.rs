// https://leetcode.com/problems/relative-sort-array/
// 2024/06/11

// use std::collections::HashMap;

// impl Solution {
//     pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
//         let n = arr1.len();
//         let mut result: Vec<i32> = Vec::with_capacity(n);

//         let mut arr1_count: HashMap<i32, usize> = HashMap::with_capacity(n);
//         for n in arr1 {
//             *arr1_count.entry(n).or_insert(0) += 1;
//         }

//         for n in arr2 {
//             for _ in 0..*arr1_count.get(&n).unwrap() {
//                 result.push(n);
//             }

//             arr1_count.remove(&n);
//         }

//         let mut result2: Vec<i32> = Vec::with_capacity(arr1_count.len());
//         for (n, count) in arr1_count {
//             for _ in 0..count {
//                 result2.push(n);
//             }
//         }
//         result2.sort_unstable();

//         result.append(&mut result2);

//         return result;
//     }
// }

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let n = arr1.len();
        let mut position = [n; 1001];

        for (i, &num) in arr2.iter().enumerate() {
            position[num as usize] = i;
        }

        arr1.sort_unstable_by_key(|&num| (position[num as usize], num));

        return arr1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::relative_sort_array(
            [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19].to_vec(),
            [2, 1, 4, 3, 9, 6].to_vec()
        )
    ); // [2,2,2,1,4,3,3,9,6,7,19]

    println!(
        "{:?}",
        Solution::relative_sort_array([28, 6, 22, 8, 44, 17].to_vec(), [22, 28, 8, 6].to_vec())
    ); // [22,28,8,6,17,44]
}

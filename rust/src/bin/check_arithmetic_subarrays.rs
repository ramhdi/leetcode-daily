// https://leetcode.com/problems/arithmetic-subarrays
// 2023/11/23

// use std::collections::HashSet;

fn is_arithmetic(nums: Vec<i32>) -> bool {
    let nums_sorted = {
        let mut sorted = nums.clone();
        sorted.sort();
        sorted
    };

    let diff = nums_sorted[1] - nums_sorted[0];
    for i in 2..nums_sorted.len() {
        if nums_sorted[i] - nums_sorted[i - 1] != diff {
            return false;
        }
    }
    return true;
}

// fn is_arithmetic(nums: Vec<i32>) -> bool {
//     let max_value = nums.iter().cloned().max().unwrap();
//     let min_value = nums.iter().cloned().min().unwrap();
//     if (max_value - min_value) % (nums.len() as i32 - 1) != 0 {
//         return false;
//     }

//     let mut nums_set: HashSet<i32> = HashSet::new();
//     for &n in &nums {
//         nums_set.insert(n);
//     }

//     let diff = (max_value - min_value) / (nums.len() as i32 - 1);
//     let mut curr = min_value + diff;
//     while curr < max_value {
//         if nums_set.get(&curr).is_none() {
//             return false;
//         }

//         curr += diff;
//     }

//     return true;
// }

fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();

    for i in 0..l.len() {
        let skip = l[i as usize] as usize;
        let take = (r[i as usize] - l[i as usize] + 1) as usize;
        result.push(is_arithmetic(
            nums.iter().cloned().skip(skip).take(take).collect(),
        ));
    }
    return result;
}

fn main() {
    println!(
        "{:?}",
        check_arithmetic_subarrays(
            vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
            vec![0, 1, 6, 4, 8, 7],
            vec![4, 4, 9, 7, 9, 10]
        ),
    );
}

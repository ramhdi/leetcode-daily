// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array
// 2023/11/17

fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let mut nums_sorted: Vec<i32> = nums.clone();
    nums_sorted.sort_unstable();
    let n = nums_sorted.len();

    for i in 0..(n / 2) {
        let sum = nums_sorted.get(i).unwrap() + nums_sorted.get(n - 1 - i).unwrap();
        if sum > result {
            result = sum;
        }
    }
    return result;
}

fn main() {
    println!("{}", min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
}

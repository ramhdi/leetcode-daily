// https://leetcode.com/problems/max-chunks-to-make-sorted/
// 2024/12/19

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .enumerate()
            .fold((0, 0), |(mut chunks, mut max_num), (i, num)| {
                max_num = max_num.max(num);
                if max_num == i as i32 {
                    chunks += 1;
                }

                (chunks, max_num)
            })
            .0
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::max_chunks_to_sorted([4, 3, 2, 1, 0].to_vec())
    ); // 1

    println!(
        "{:?}",
        Solution::max_chunks_to_sorted([1, 0, 2, 3, 4].to_vec())
    ); // 4
}

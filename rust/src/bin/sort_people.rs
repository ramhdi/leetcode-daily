// https://leetcode.com/problems/sort-the-people/
// 2024/07/22

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut pairs: Vec<(String, i32)> = names.into_iter().zip(heights).collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        pairs.into_iter().map(|(name, _)| name).collect()
    }
}
struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::sort_people(
            ["Mary", "John", "Emma"].map(|s| s.to_string()).to_vec(),
            [180, 165, 170].to_vec()
        )
    ); // ["Mary","Emma","John"]

    println!(
        "{:?}",
        Solution::sort_people(
            ["Alice", "Bob", "Bob"].map(|s| s.to_string()).to_vec(),
            [155, 185, 150].to_vec()
        )
    ); // ["Bob","Alice","Bob"]
}

// https://leetcode.com/problems/most-beautiful-item-for-each-query/
// 2024/11/12

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|item| item[0]);

        let mut prices = Vec::new();
        let mut max_beauty = Vec::new();
        let mut current_max = 0;

        for item in items {
            current_max = current_max.max(item[1]);
            if prices.is_empty() || prices.last() != Some(&item[0]) {
                prices.push(item[0]);
                max_beauty.push(current_max);
            } else {
                *max_beauty.last_mut().unwrap() = current_max;
            }
        }

        queries
            .into_iter()
            .map(|q| match prices.binary_search(&q) {
                Ok(i) => max_beauty[i],
                Err(i) if i > 0 => max_beauty[i - 1],
                _ => 0,
            })
            .collect()
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::maximum_beauty(
            [[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]]
                .map(|e| e.to_vec())
                .to_vec(),
            [1, 2, 3, 4, 5, 6].to_vec()
        )
    ); // [2,4,5,5,6,6]

    println!(
        "{:?}",
        Solution::maximum_beauty(
            [[1, 2], [1, 2], [1, 3], [1, 4]]
                .map(|e| e.to_vec())
                .to_vec(),
            [1].to_vec()
        )
    ); // [4]

    println!(
        "{:?}",
        Solution::maximum_beauty([[10, 1000]].map(|e| e.to_vec()).to_vec(), [5].to_vec())
    ); // [0]
}

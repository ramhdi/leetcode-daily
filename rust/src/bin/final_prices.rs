// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
// 2024/12/18

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        prices
            .iter()
            .enumerate()
            .map(|(i, &current)| {
                let discount = prices
                    .iter()
                    .skip(i + 1)
                    .find(|&&x| x <= current)
                    .copied()
                    .unwrap_or(0);

                current - discount
            })
            .collect()
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::final_prices([8, 4, 6, 2, 3].to_vec())); // [4,2,4,2,3]

    println!("{:?}", Solution::final_prices([1, 2, 3, 4, 5].to_vec())); // [1,2,3,4,5]

    println!("{:?}", Solution::final_prices([10, 1, 1, 6].to_vec())); // [9,0,1,6]
}

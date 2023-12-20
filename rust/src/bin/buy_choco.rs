// https://leetcode.com/problems/buy-two-chocolates/
// 2023-12-20

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut cheapest: i32 = i32::MAX;
        let mut second_cheapest: i32 = i32::MAX;

        for price in prices {
            if price < cheapest {
                second_cheapest = cheapest;
                cheapest = price;
            } else {
                second_cheapest = std::cmp::min(second_cheapest, price);
            }
        }

        if money - (cheapest + second_cheapest) < 0 {
            return money;
        }
        return money - (cheapest + second_cheapest);
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::buy_choco(vec![1, 2, 2], 3));
    println!("{:?}", Solution::buy_choco(vec![3, 2, 3], 3));
}

// https://leetcode.com/problems/cheapest-flights-within-k-stops/
// 2024/02/23

use std::collections::HashMap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut costs = vec![u16::MAX; n as usize];
        costs[src as usize] = 0;

        let mut new_costs = Vec::with_capacity(n as usize);
        for _ in 0..=k {
            new_costs.clear();
            new_costs.extend_from_slice(&costs[..]);
            for flight in flights.iter() {
                let (from, to, price) = (flight[0] as usize, flight[1] as usize, flight[2] as u16);
                if costs[from] != u16::MAX {
                    new_costs[to] = new_costs[to].min(costs[from] + price);
                }
            }
            std::mem::swap(&mut costs, &mut new_costs);
        }

        match costs[dst as usize] {
            u16::MAX => -1,
            res => res as i32,
        }
    }

    pub fn _find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut flights_from: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        for f in flights {
            flights_from
                .entry(f[0])
                .or_insert(Vec::new())
                .push((f[1], f[2]));
        }

        if flights_from.get(&src).is_none() {
            return -1;
        }

        let mut current_dest_cost: Vec<(i32, i32)> = flights_from.get(&src).unwrap().clone();
        for _ in 0..k {
            let mut new_dest_cost: Vec<(i32, i32)> = Vec::new();

            for dest_cost in current_dest_cost {
                if dest_cost.0 == dst {
                    new_dest_cost.push(dest_cost);
                } else {
                    if let Some(dests_costs) = flights_from.get(&dest_cost.0) {
                        for &dc in dests_costs {
                            new_dest_cost.push((dc.0, dc.1 + dest_cost.1));
                        }
                    }
                }
            }

            current_dest_cost = new_dest_cost;
        }

        let mut result: i32 = i32::MAX;
        for dc in current_dest_cost {
            if dc.0 == dst {
                result = std::cmp::min(result, dc.1);
            }
        }

        if result == i32::MAX {
            return -1;
        }

        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_cheapest_price(
            4,
            [
                [0, 1, 100],
                [1, 2, 100],
                [2, 0, 100],
                [1, 3, 600],
                [2, 3, 200]
            ]
            .map(|f| f.to_vec())
            .to_vec(),
            0,
            3,
            1
        )
    ); // 700

    println!(
        "{:?}",
        Solution::find_cheapest_price(
            3,
            [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
                .map(|f| f.to_vec())
                .to_vec(),
            0,
            2,
            1
        )
    ); // 200

    println!(
        "{:?}",
        Solution::find_cheapest_price(
            3,
            [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
                .map(|f| f.to_vec())
                .to_vec(),
            0,
            2,
            0
        )
    ); // 500

    println!(
        "{:?}",
        Solution::find_cheapest_price(
            17,
            [
                [0, 12, 28],
                [5, 6, 39],
                [8, 6, 59],
                [13, 15, 7],
                [13, 12, 38],
                [10, 12, 35],
                [15, 3, 23],
                [7, 11, 26],
                [9, 4, 65],
                [10, 2, 38],
                [4, 7, 7],
                [14, 15, 31],
                [2, 12, 44],
                [8, 10, 34],
                [13, 6, 29],
                [5, 14, 89],
                [11, 16, 13],
                [7, 3, 46],
                [10, 15, 19],
                [12, 4, 58],
                [13, 16, 11],
                [16, 4, 76],
                [2, 0, 12],
                [15, 0, 22],
                [16, 12, 13],
                [7, 1, 29],
                [7, 14, 100],
                [16, 1, 14],
                [9, 6, 74],
                [11, 1, 73],
                [2, 11, 60],
                [10, 11, 85],
                [2, 5, 49],
                [3, 4, 17],
                [4, 9, 77],
                [16, 3, 47],
                [15, 6, 78],
                [14, 1, 90],
                [10, 5, 95],
                [1, 11, 30],
                [11, 0, 37],
                [10, 4, 86],
                [0, 8, 57],
                [6, 14, 68],
                [16, 8, 3],
                [13, 0, 65],
                [2, 13, 6],
                [5, 13, 5],
                [8, 11, 31],
                [6, 10, 20],
                [6, 2, 33],
                [9, 1, 3],
                [14, 9, 58],
                [12, 3, 19],
                [11, 2, 74],
                [12, 14, 48],
                [16, 11, 100],
                [3, 12, 38],
                [12, 13, 77],
                [10, 9, 99],
                [15, 13, 98],
                [15, 12, 71],
                [1, 4, 28],
                [7, 0, 83],
                [3, 5, 100],
                [8, 9, 14],
                [15, 11, 57],
                [3, 6, 65],
                [1, 3, 45],
                [14, 7, 74],
                [2, 10, 39],
                [4, 8, 73],
                [13, 5, 77],
                [10, 0, 43],
                [12, 9, 92],
                [8, 2, 26],
                [1, 7, 7],
                [9, 12, 10],
                [13, 11, 64],
                [8, 13, 80],
                [6, 12, 74],
                [9, 7, 35],
                [0, 15, 48],
                [3, 7, 87],
                [16, 9, 42],
                [5, 16, 64],
                [4, 5, 65],
                [15, 14, 70],
                [12, 0, 13],
                [16, 14, 52],
                [3, 10, 80],
                [14, 11, 85],
                [15, 2, 77],
                [4, 11, 19],
                [2, 7, 49],
                [10, 7, 78],
                [14, 6, 84],
                [13, 7, 50],
                [11, 6, 75],
                [5, 10, 46],
                [13, 8, 43],
                [9, 10, 49],
                [7, 12, 64],
                [0, 10, 76],
                [5, 9, 77],
                [8, 3, 28],
                [11, 9, 28],
                [12, 16, 87],
                [12, 6, 24],
                [9, 15, 94],
                [5, 7, 77],
                [4, 10, 18],
                [7, 2, 11],
                [9, 5, 41]
            ]
            .map(|f| f.to_vec())
            .to_vec(),
            13,
            4,
            13
        )
    ); // 47

    println!(
        "{:?}",
        Solution::find_cheapest_price(
            7,
            [
                [0, 1, 100],
                [0, 2, 100],
                [2, 3, 1],
                [3, 4, 600],
                [1, 4, 602],
                [4, 5, 6],
                [5, 6, 6]
            ]
            .map(|f| f.to_vec())
            .to_vec(),
            0,
            6,
            3
        )
    ); // 714
}

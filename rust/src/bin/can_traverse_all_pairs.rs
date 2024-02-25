// https://leetcode.com/problems/greatest-common-divisor-traversal/
// 2024/02/25

use std::collections::HashMap;

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let num_elements = nums.len();
        if num_elements == 1 {
            return true;
        }

        let mut disjoint_set: Vec<usize> = (0..num_elements).collect();
        let mut set_size: Vec<usize> = vec![1; num_elements];
        let mut factor_first_occurrence: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let mut num = num;
            let mut divisor = 2;

            while divisor * divisor <= num {
                if num % divisor == 0 {
                    let factor = divisor;
                    if let Some(&first_occurrence) = factor_first_occurrence.get(&factor) {
                        Solution::union_sets(&mut disjoint_set, &mut set_size, i, first_occurrence);
                    } else {
                        factor_first_occurrence.insert(factor, i);
                    }

                    while num % factor == 0 {
                        num /= factor;
                    }
                }
                divisor += 1;
            }

            if num > 1 {
                if let Some(&first_occurrence) = factor_first_occurrence.get(&num) {
                    Solution::union_sets(&mut disjoint_set, &mut set_size, i, first_occurrence);
                } else {
                    factor_first_occurrence.insert(num, i);
                }
            }
        }

        let leader = Solution::find_set_leader(&mut disjoint_set, 0);
        set_size[leader] == num_elements
    }

    fn find_set_leader(disjoint_set: &mut Vec<usize>, x: usize) -> usize {
        if disjoint_set[x] == x {
            return x;
        }
        disjoint_set[x] = Solution::find_set_leader(disjoint_set, disjoint_set[x]);
        disjoint_set[x]
    }

    fn union_sets(disjoint_set: &mut Vec<usize>, set_size: &mut Vec<usize>, x: usize, y: usize) {
        let x_leader = Solution::find_set_leader(disjoint_set, x);
        let y_leader = Solution::find_set_leader(disjoint_set, y);
        if x_leader == y_leader {
            return;
        }
        if set_size[x_leader] < set_size[y_leader] {
            disjoint_set[x_leader] = y_leader;
            set_size[y_leader] += set_size[x_leader];
        } else {
            disjoint_set[y_leader] = x_leader;
            set_size[x_leader] += set_size[y_leader];
        }
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::can_traverse_all_pairs([2, 3, 6].to_vec(),)
    ); // true
    println!(
        "{:?}",
        Solution::can_traverse_all_pairs([3, 9, 5].to_vec(),)
    ); // false
    println!(
        "{:?}",
        Solution::can_traverse_all_pairs([4, 3, 12, 8].to_vec(),)
    ); // true
}

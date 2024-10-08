// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/
// 2024/10/04

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        skill.sort_unstable();
        let mut chemistry = 0;
        let mut total_skill = skill[0] + skill[n - 1];
        chemistry += (skill[0] * skill[n - 1]) as i64;
        for i in 1..=(n / 2 - 1) {
            if skill[i] + skill[n - 1 - i] != total_skill {
                return -1;
            }

            chemistry += (skill[i] * skill[n - 1 - i]) as i64;
            total_skill = skill[i] + skill[n - 1 - i];
        }

        chemistry
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::divide_players([3, 1, 4, 2].to_vec())); // 22

    println!("{:?}", Solution::divide_players([3, 4].to_vec())); // 12

    println!("{:?}", Solution::divide_players([1, 1, 2, 3].to_vec())); // -1
}

// https://leetcode.com/problems/find-the-town-judge/
// 2024/02/22

use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.len() == 0 {
            return 1;
        }

        let mut trust_count: HashMap<i32, u16> = HashMap::new();

        for t in trust.clone() {
            *trust_count.entry(t[1]).or_insert(0) += 1;
        }

        for (p, c) in trust_count {
            if c == n as u16 - 1 {
                for t in trust {
                    if t[0] == p {
                        return -1;
                    }
                }
                return p;
            }
        }

        return -1;
    }

    pub fn _find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }

        let n = n as usize;
        let mut in_trust = vec![0 as u16; n];
        let mut out_trust = vec![0 as u16; n];
        for t in trust {
            in_trust[t[1] as usize - 1] += 1;
            out_trust[t[0] as usize - 1] += 1;
        }

        for i in 0..n {
            if in_trust[i] == n as u16 - 1 && out_trust[i] == 0 {
                return i as i32 + 1;
            }
        }

        return -1;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_judge(2, [[1, 2]].map(|t| t.to_vec()).to_vec())
    ); // 2

    println!(
        "{:?}",
        Solution::find_judge(3, [[1, 3], [2, 3]].map(|t| t.to_vec()).to_vec())
    ); // 3

    println!(
        "{:?}",
        Solution::find_judge(3, [[1, 3], [2, 3], [3, 1]].map(|t| t.to_vec()).to_vec())
    ); // -1
}

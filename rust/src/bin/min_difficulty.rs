// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/?
// 2023-12-29

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        if d > n {
            return -1;
        }
        let l = n - d + 1;
        let mut m = 0;
        let mut dp: Vec<i32> = job_difficulty
            .iter()
            .take(l)
            .map(|v| {
                m = m.max(*v);
                m
            })
            .collect();
        for i in 1..d {
            let mut dpn = Vec::with_capacity(l);
            for j in 0..l {
                let mut m = job_difficulty[i + j];
                let mut r = m + dp[j];
                for k in (0..j).rev() {
                    m = m.max(job_difficulty[i + k]);
                    r = r.min(m + dp[k]);
                }
                dpn.push(r);
            }
            dp = dpn;
        }
        dp[l - 1]
    }

    pub fn _min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);

        if n < d {
            return -1;
        }

        let (mut dp_1, mut dp_2) = (vec![0; n], vec![0; n]);
        dp_1[n - 1] = job_difficulty[n - 1];
        for i in (0..n - 1).rev() {
            dp_1[i] = dp_1[i + 1].max(job_difficulty[i]);
        }

        for days in 2..d + 1 {
            for i in 0..n - days + 1 {
                let mut max_dif = i32::MIN;
                let mut min = i32::MAX;
                for j in i..n - days + 1 {
                    max_dif = max_dif.max(job_difficulty[j]);
                    min = min.min(max_dif + dp_1[j + 1]);
                }
                dp_2[i] = min;
            }
            std::mem::swap(&mut dp_1, &mut dp_2);
        }

        dp_1[0]
    }

    pub fn __min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let n = job_difficulty.len();
        if n < d {
            return -1;
        }
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![i32::MAX; d]; n]; n];
        for id in 0..d {
            for start in 0..n {
                for end in start..n {
                    if id == 0 {
                        dp[start][end][id] =
                            job_difficulty[start..=end].iter().cloned().max().unwrap();
                    } else if end - start >= id {
                        let mut result = dp[start][end][id];
                        for i in start..=(end - id) {
                            result =
                                std::cmp::min(result, dp[start][i][0] + dp[i + 1][end][id - 1]);
                        }
                        if result == i32::MAX {
                            dp[start][end][id] = -1;
                        } else {
                            dp[start][end][id] = result;
                        }
                    }
                }
            }
        }
        if dp[0][n - 1][d - 1] == i32::MAX {
            return -1;
        }
        return dp[0][n - 1][d - 1];
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2));
    println!("{:?}", Solution::min_difficulty(vec![9, 9, 9], 4));
    println!("{:?}", Solution::min_difficulty(vec![1, 1, 1], 3));
    println!("{:?}", Solution::min_difficulty(vec![7, 1, 7, 1, 7, 1], 3));
    println!(
        "{:?}",
        Solution::min_difficulty(
            vec![
                380, 302, 102, 681, 863, 676, 243, 671, 651, 612, 162, 561, 394, 856, 601, 30, 6,
                257, 921, 405, 716, 126, 158, 476, 889, 699, 668, 930, 139, 164, 641, 801, 480,
                756, 797, 915, 275, 709, 161, 358, 461, 938, 914, 557, 121, 964, 315
            ],
            10
        )
    );
}

// https://leetcode.com/problems/find-all-people-with-secret/
// 2024/02/24

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut know_secret = vec![false; n as usize];

        know_secret[0] = true;
        know_secret[first_person as usize] = true;

        let mut meetings = meetings;
        meetings.sort_unstable_by_key(|x| x[2]);

        let mut left = 0;
        while left < meetings.len() {
            let time = meetings[left][2];
            let mut secret_shared = true;
            let mut right = left;
            while secret_shared {
                secret_shared = false;
                right = left;
                while right < meetings.len() && meetings[right][2] == time {
                    let x = meetings[right][0] as usize;
                    let y = meetings[right][1] as usize;
                    if know_secret[x] && !know_secret[y] || !know_secret[x] && know_secret[y] {
                        secret_shared = true;
                        know_secret[x] = true;
                        know_secret[y] = true;
                    }
                    right += 1;
                }
            }
            left = right;
        }
        know_secret
            .into_iter()
            .enumerate()
            .filter(|x| x.1)
            .map(|x| x.0 as i32)
            .collect()
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_all_people(
            6,
            [[1, 2, 5], [2, 3, 8], [1, 5, 10]]
                .map(|f| f.to_vec())
                .to_vec(),
            1
        )
    ); // [0,1,2,3,5]

    println!(
        "{:?}",
        Solution::find_all_people(
            4,
            [[3, 1, 3], [1, 2, 2], [0, 3, 3]]
                .map(|f| f.to_vec())
                .to_vec(),
            3
        )
    ); // [0,1,3]

    println!(
        "{:?}",
        Solution::find_all_people(
            5,
            [[3, 4, 2], [1, 2, 1], [2, 3, 1]]
                .map(|f| f.to_vec())
                .to_vec(),
            1
        )
    ); // [0,1,2,3,4]
}

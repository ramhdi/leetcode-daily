// https://leetcode.com/problems/number-of-senior-citizens/
// 2024/08/01

// impl Solution {
//     pub fn count_seniors(details: Vec<String>) -> i32 {
//         let mut seniors: i32 = 0;

//         for detail in details {
//             let d = detail.as_bytes();
//             let age = 10 * (d[11] as i32 - 48) + (d[12] as i32 - 48);
//             if age > 60 {
//                 seniors += 1;
//             }
//         }

//         return seniors;
//     }
// }

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|detail| {
                let age = (&detail[11..13]).parse::<u8>().unwrap();
                age > 60
            })
            .count() as i32
    }
}

struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::count_seniors(
            ["7868190130M7522", "5303914400F9211", "9273338290F4010"]
                .map(|p| p.to_string())
                .to_vec()
        )
    ); // 2

    println!(
        "{}",
        Solution::count_seniors(
            ["1313579440F2036", "2921522980M5644"]
                .map(|p| p.to_string())
                .to_vec()
        )
    ); // 0
}

// https://leetcode.com/problems/find-common-characters/
// 2024/06/05

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut ans = [u16::MAX; 26];
        words.iter().for_each(|word| {
            let mut count = [0; 26];
            word.chars()
                .for_each(|c| count[(c as u8 - b'a') as usize] += 1);
            ans.iter_mut()
                .zip(count.iter())
                .for_each(|(a, &c)| *a = (*a).min(c));
        });

        (0..26)
            .flat_map(|i| {
                std::iter::repeat((b'a' + i as u8) as char)
                    .take(ans[i] as usize)
                    .map(|c| c.to_string())
            })
            .collect()
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::common_chars(["bella", "label", "roller"].map(|s| s.to_string()).to_vec())
    ); // ["e","l","l"]

    println!(
        "{:?}",
        Solution::common_chars(["cool", "lock", "cook"].map(|s| s.to_string()).to_vec())
    ); // ["c","o"]
}

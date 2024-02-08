// https://leetcode.com/problems/sort-characters-by-frequency/
// 2024/02/07

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut char_count = [0 as usize; (b'z' - b'0' + 1) as usize];
        let s = s.as_bytes();
        let mut result: String = String::new();

        for &c in s {
            char_count[(c - b'0') as usize] += 1;
        }

        let mut arr_index_count: Vec<(usize, usize)> = char_count
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();

        arr_index_count.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        for (i, c) in arr_index_count {
            for _ in 0..c {
                result += &((i as u8 + b'0') as char).to_string();
            }
        }

        return result;
    }
}
pub struct Solution {}

fn main() {
    println!("{:?}", Solution::frequency_sort("tree".to_string())); // "eert"
    println!("{:?}", Solution::frequency_sort("cccaaa".to_string())); // "aaaccc"
    println!("{:?}", Solution::frequency_sort("Aabb".to_string())); // "bbAa"
    println!(
        "{:?}",
        Solution::frequency_sort("2a554442f544asfasssffffasss".to_string())
    ); // "sssssssffffff44444aaaa55522"
}

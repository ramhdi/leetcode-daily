// https://leetcode.com/problems/string-compression/

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut index: usize = 1;
        let mut count: usize = 1;
        for i in 0..chars.len() {
            if i < chars.len() - 1 && chars[i] == chars[i + 1] {
                count += 1;
            } else {
                chars[index - 1] = chars[i];
                if count > 1 {
                    for d in count.to_string().chars() {
                        chars[index] = d;
                        index += 1;
                    }
                }
                if i < chars.len() - 1 {
                    index += 1;
                    count = 1;
                }
            }
        }
        println!("{:?}", chars);
        return index as i32;
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::compress(vec!['a', 'a', 'b', 'b', 'b', 'c', 'c', 'c'].as_mut())
    );
    println!("{:?}", Solution::compress(vec!['a'].as_mut()));
    println!(
        "{:?}",
        Solution::compress(
            vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'].as_mut()
        )
    );
    println!(
        "{:?}",
        Solution::compress(
            vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'c', 'c'].as_mut()
        )
    );
}

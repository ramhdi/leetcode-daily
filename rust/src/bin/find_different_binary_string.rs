// https://leetcode.com/problems/find-unique-binary-string
// 2023/11/16

fn find_different_binary_string(nums: Vec<String>) -> String {
    let mut result: String = String::new();
    for (i, num) in nums.iter().enumerate() {
        match num.chars().nth(i) {
            Some(c) => match c {
                '1' => result += "0",
                '0' => result += "1",
                _ => (),
            },
            None => (),
        }
    }
    return result;
}

fn main() {
    println!(
        "{}",
        find_different_binary_string(vec![
            "000".to_string(),
            "111".to_string(),
            "010".to_string()
        ])
    );
}

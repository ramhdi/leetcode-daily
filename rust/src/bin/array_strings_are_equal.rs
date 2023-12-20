// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent
// 2023/11/30

fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    return word1.join("") == word2.join("");
}

fn main() {
    println!(
        "{}",
        array_strings_are_equal(
            vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
            vec!["abcddefg".to_string()]
        )
    );
}

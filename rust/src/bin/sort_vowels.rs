// https://leetcode.com/problems/sort-vowels-in-a-string
// 2023/11/13

use std::collections::HashMap;

fn is_vowel(c: char) -> bool {
    return c == 'a'
        || c == 'e'
        || c == 'i'
        || c == 'o'
        || c == 'u'
        || c == 'A'
        || c == 'E'
        || c == 'I'
        || c == 'O'
        || c == 'U';
}

fn sort_vowels(s: String) -> String {
    let mut result: String = "".to_string();
    let mut char_count: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        if is_vowel(c) {
            let counter = char_count.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    let sorted_vowel = "AEIOUaeiou";
    let mut j = 0;
    for c in s.chars() {
        if !is_vowel(c) {
            result += &c.to_string();
        } else {
            while char_count
                .get(&sorted_vowel.chars().nth(j).unwrap())
                .unwrap_or(&0)
                == &0
            {
                j += 1;
            }

            result += &sorted_vowel.chars().nth(j).unwrap().to_string();

            let counter = char_count
                .entry(sorted_vowel.chars().nth(j).unwrap())
                .or_insert(0);
            *counter -= 1;
        }
    }
    return result;
}

fn main() {
    println!("{}", sort_vowels("lEetcOdeaaaaa".to_string()));
}

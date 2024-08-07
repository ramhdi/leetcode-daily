// https://leetcode.com/problems/integer-to-english-words/
// 2024/08/07

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let ones = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let twos = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        let mut exponents = ["Thousand", "Million", "Billion"];

        let mut result: String = String::new();

        let part1 = num / 1_000_000_000;
        let remainder1 = num % 1_000_000_000;

        let part2 = remainder1 / 1_000_000;
        let remainder2 = remainder1 % 1_000_000;

        let part3 = remainder2 / 1_000;
        let part4 = remainder2 % 1_000;

        if part1 > 0 {
            result += &Self::helper(part1, &ones, &twos);
            result += " Billion";
        }

        if part2 > 0 {
            if result.len() > 0 {
                result += " ";
            }
            result += &Self::helper(part2, &ones, &twos);
            result += " Million";
        }

        if part3 > 0 {
            if result.len() > 0 {
                result += " ";
            }
            result += &Self::helper(part3, &ones, &twos);
            result += " Thousand";
        }

        if part4 > 0 {
            if result.len() > 0 {
                result += " ";
            }
            result += &Self::helper(part4, &ones, &twos);
        }

        result
    }

    fn helper(num: i32, ones: &[&str], twos: &[&str]) -> String {
        let mut result: String = String::new();
        let hundreds = (num / 100) as usize;
        let remainders = (num % 100) as usize;

        if hundreds > 0 && hundreds < 10 {
            result += ones[hundreds];
            result += " Hundred";
        }

        if remainders > 0 && remainders < 100 {
            if result.len() > 0 {
                result += " ";
            }
            if remainders < 20 {
                result += ones[remainders];
            } else {
                let tens = (remainders / 10) as usize;
                let singles = (remainders % 10) as usize;

                result += twos[tens];

                if singles > 0 && singles < 10 {
                    result += " ";
                    result += ones[singles];
                }
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    // println!("{}", Solution::number_to_words(717));

    println!("{}", Solution::number_to_words(123)); // "One Hundred Twenty Three"

    println!("{}", Solution::number_to_words(12345)); // "Twelve Thousand Three Hundred Forty Five"

    println!("{}", Solution::number_to_words(1234567)); // "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"

    println!("{}", Solution::number_to_words(2147483647)); // "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven"
}

// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage
// 2023/11/20

fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let garbage_types: Vec<char> = vec!['M', 'P', 'G'];
    let mut last_house: Vec<usize> = vec![0, 0, 0];

    for (i, &t) in garbage_types.iter().enumerate() {
        for (j, g) in garbage.iter().enumerate() {
            match g.find(t) {
                Some(_) => last_house[i] = j,
                None => (),
            }
        }

        for k in 0..=last_house[i] {
            let g: &String = &garbage[k];
            for gt in g.chars() {
                if gt == t {
                    result += 1;
                }
            }

            if k < last_house[i] {
                result += travel[k];
            }
        }
    }
    return result;
}

fn main() {
    println!(
        "{}",
        garbage_collection(
            vec![
                "G".to_string(),
                "P".to_string(),
                "GP".to_string(),
                "GG".to_string(),
            ],
            vec![2, 4, 3]
        )
    );
}

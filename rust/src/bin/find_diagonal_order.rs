// https://leetcode.com/problems/diagonal-traverse-ii
// 2023/11/22

fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut count: Vec<(usize, usize, i32)> = Vec::new();
    for (i, row) in nums.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            count.push((i + j, j, num));
        }
    }

    count.sort_by(|a, b| {
        let cmp_first = a.0.cmp(&b.0);
        if cmp_first.is_eq() {
            return a.1.cmp(&b.1);
        }
        return cmp_first;
    });

    for (_, _, num) in count {
        result.push(num);
    }
    return result;
}

fn main() {
    println!(
        "{:?}",
        find_diagonal_order(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16]
        ],)
    );
}

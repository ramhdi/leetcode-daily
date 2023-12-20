// https://leetcode.com/problems/image-smoother/
// 2023/12/19

impl Solution {
    pub fn image_smoother(mut img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();

        for i in 0..m {
            for j in 0..n {
                let mut count: i32 = 0;
                let mut sum: i32 = 0;
                for a in (i.max(1) - 1)..=((i as i32).min((m as i32) - 2) + 1) as usize {
                    for b in (j.max(1) - 1)..=((j as i32).min((n as i32) - 2) + 1) as usize {
                        sum += img[a][b] & 255;
                        count += 1;
                    }
                }
                img[i][j] |= (sum / count) << 8;
            }
        }

        for i in 0..m {
            for j in 0..n {
                img[i][j] >>= 8;
            }
        }

        return img;
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
    );
    println!(
        "{:?}",
        Solution::image_smoother(vec![
            vec![100, 200, 100],
            vec![200, 50, 200],
            vec![100, 200, 100]
        ])
    );
    println!("{:?}", Solution::image_smoother(vec![vec![1]]));
}

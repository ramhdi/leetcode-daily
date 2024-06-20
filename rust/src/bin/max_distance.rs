// https://leetcode.com/problems/magnetic-force-between-two-balls/
// 2024/06/20

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let (mut min_dist, mut max_dist) =
            (1, position.last().unwrap() - position.first().unwrap());

        while min_dist <= max_dist {
            let dist = min_dist + (max_dist - min_dist) / 2;
            if Self::can_place_balls(dist, m, &position) {
                min_dist = dist + 1;
            } else {
                max_dist = dist - 1;
            }
        }

        return max_dist;
    }

    fn can_place_balls(curr_dist: i32, num_balls: i32, position: &Vec<i32>) -> bool {
        let mut placed_balls = 1;
        let mut last_ball_place = position[0];

        for &p in position.iter().skip(1) {
            if p - last_ball_place >= curr_dist {
                placed_balls += 1;
                last_ball_place = p;
                if placed_balls >= num_balls {
                    return true;
                }
            }
        }

        return false;
    }
}

pub struct Solution {}

fn main() {
    println!("{:?}", Solution::max_distance([1, 2, 3, 4, 7].to_vec(), 3)); // 3

    println!(
        "{:?}",
        Solution::max_distance([5, 4, 3, 2, 1, 1000000000].to_vec(), 2)
    ); // 999999999
}

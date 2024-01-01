// https://leetcode.com/problems/assign-cookies/
// 2024/01/01

impl Solution {
    pub fn find_content_children(mut child_greed: Vec<i32>, mut cookie_size: Vec<i32>) -> i32 {
        let mut ig: usize = 0;
        let mut is: usize = 0;
        let ng = child_greed.len();
        let ns = cookie_size.len();
        child_greed.sort();
        cookie_size.sort();
        while is < ns && ig < ng {
            if child_greed[ig] <= cookie_size[is] {
                ig += 1;
            }
            is += 1;
        }
        return ig as i32;
    }
}
pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
    );
    println!(
        "{:?}",
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
    );
    println!(
        "{:?}",
        Solution::find_content_children(vec![7, 8, 9, 10], vec![5, 6, 7, 8])
    );
}

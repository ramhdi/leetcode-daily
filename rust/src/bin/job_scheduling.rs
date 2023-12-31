// https://leetcode.com/problems/maximum-profit-in-job-scheduling/
// 2024/01/06

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = profit.len();
        let mut jobs: Vec<(i32, i32, i32)> = Vec::new();
        let mut dp: Vec<(i32, i32)> = Vec::new();

        for i in 0..n {
            jobs.push((start_time[i], end_time[i], profit[i]));
        }

        jobs.sort_by(|j1, j2| j1.0.cmp(&j2.0));
        for i in (0..n).rev() {
            dp.push((jobs[i].0, jobs[i].2));
            if i != n - 1 {
                let mut ndp: Vec<(i32, i32)> = Vec::new();
                for d in dp {
                    ndp.push(d);
                    if jobs[i].1 <= d.0 {
                        let nd: (i32, i32) = (jobs[i].0, d.1 + jobs[i].2);
                        ndp.push(nd);
                    }
                }
                dp = ndp;
            }
        }

        let mut result: i32 = 0;
        for d in dp {
            result = std::cmp::max(result, d.1);
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
    ); // 120
    println!(
        "{:?}",
        Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60]
        )
    ); // 150
    println!(
        "{:?}",
        Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4])
    ); // 6
    println!(
        "{:?}",
        Solution::job_scheduling(
            vec![6, 15, 7, 11, 1, 3, 16, 2],
            vec![19, 18, 19, 16, 10, 8, 19, 8],
            vec![2, 9, 1, 19, 5, 7, 3, 19]
        )
    ); // 41
    println!(
        "{:?}",
        Solution::job_scheduling(
            vec![
                631, 919, 696, 968, 618, 133, 263, 517, 265, 290, 741, 646, 534, 605, 978, 584,
                937, 37, 666, 446, 264, 58, 461, 648, 382, 783, 719, 958, 247, 837, 547, 978, 169,
                172, 545, 326, 720, 232, 121, 335, 575, 496, 701, 662, 201, 641, 158, 976, 658,
                888, 645, 338, 401, 627, 803, 716, 139, 243, 382, 592, 287, 743, 683, 162, 220,
                871, 957, 694, 108, 318, 390, 416, 855, 922, 293, 116, 574, 759, 50, 690, 314, 424,
                607, 894, 520, 972, 85, 214, 118, 992, 197, 865, 826, 160, 19, 583, 520, 585, 268,
                872
            ],
            vec![
                811, 960, 887, 986, 685, 440, 339, 709, 682, 510, 897, 896, 588, 906, 980, 604,
                984, 676, 788, 748, 814, 207, 852, 905, 478, 880, 732, 986, 327, 864, 739, 990,
                221, 354, 594, 763, 962, 273, 139, 416, 852, 887, 809, 959, 718, 919, 175, 994,
                897, 987, 651, 530, 939, 819, 807, 874, 956, 651, 809, 952, 442, 861, 990, 535,
                732, 926, 965, 900, 195, 595, 492, 432, 950, 963, 857, 280, 712, 794, 751, 732,
                754, 531, 710, 958, 694, 982, 884, 352, 729, 996, 253, 947, 940, 268, 442, 763,
                963, 862, 760, 884
            ],
            vec![
                7, 25, 35, 83, 75, 89, 61, 23, 28, 97, 43, 100, 92, 29, 97, 44, 52, 55, 91, 18, 27,
                7, 34, 41, 11, 12, 20, 89, 50, 96, 80, 36, 90, 79, 91, 18, 12, 50, 95, 32, 78, 66,
                17, 59, 60, 39, 18, 75, 73, 75, 60, 49, 75, 86, 67, 10, 76, 6, 40, 81, 35, 93, 29,
                96, 94, 92, 99, 3, 76, 88, 97, 64, 79, 39, 5, 81, 46, 82, 82, 86, 43, 54, 36, 35,
                90, 26, 65, 59, 22, 44, 84, 14, 97, 99, 81, 35, 41, 15, 82, 30
            ]
        )
    ); // 41
}

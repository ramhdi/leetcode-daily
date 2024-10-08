// https://leetcode.com/problems/kth-largest-element-in-a-stream/
// 2024/08/12

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let heap = BinaryHeap::with_capacity(k as usize);
        let mut kth_largest = KthLargest {
            k: k as usize,
            heap,
        };

        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }

        self.heap.peek().unwrap().0
    }
}

struct Solution {}

fn main() {}

// https://leetcode.com/problems/insert-delete-getrandom-o1/
// 2023-11-16

use rand::prelude::*;
use std::collections::HashSet;

pub struct RandomizedSet {
    set: HashSet<i32>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        return Self {
            set: HashSet::new(),
            vec: Vec::new(),
        };
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.set.contains(&val) {
            return false;
        }

        self.set.insert(val);
        self.vec.push(val);
        return true;
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.set.contains(&val) {
            return false;
        }

        self.set.remove(&val);
        if let Some(last_index) = self.vec.iter().position(|&x| x == val) {
            self.vec.swap_remove(last_index);
        }
        return true;
    }

    pub fn get_random(&self) -> i32 {
        let index = thread_rng().gen_range(0, self.vec.len());
        return self.vec[index];
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    println!("{:?}", obj.insert(1));
    println!("{:?}", obj.remove(2));
    println!("{:?}", obj.insert(2));
    println!("{:?}", obj.get_random());
    println!("{:?}", obj.remove(1));
    println!("{:?}", obj.insert(2));
    println!("{:?}", obj.get_random());

    // println!("{:?}", obj.insert(1));
    // println!("{:?}", obj.insert(10));
    // println!("{:?}", obj.insert(20));
    // println!("{:?}", obj.insert(30));
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
    // println!("{:?}", obj.get_random());
}

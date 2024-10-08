// https://leetcode.com/problems/my-calendar-i/
// 2024/09/26

use std::collections::BTreeSet;

impl Solution {}
pub struct Solution {}

struct MyCalendar {
    bookings: BTreeSet<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        return MyCalendar {
            bookings: BTreeSet::new(),
        };
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(&(_, e)) = self.bookings.range(..=(start, end)).rev().next() {
            if e > start {
                return false;
            }
        }

        if let Some(&(s, _)) = self.bookings.range((start, end)..).next() {
            if s < end {
                return false;
            }
        }

        self.bookings.insert((start, end));
        true
    }

    fn peek(&self) {
        for &b in &self.bookings {
            println!("{:?}", b);
        }
    }
}

fn main() {
    let mut calendar = MyCalendar::new();
    let inputs = [[10, 20], [15, 25], [20, 30]];
    let outputs = inputs.map(|e| calendar.book(e[0], e[1]));

    println!("{:?}", outputs); // [true, false, true]]
    calendar.peek();
}

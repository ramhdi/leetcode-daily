// https://leetcode.com/problems/design-a-stack-with-increment-operation/
// 2024/09/30

impl Solution {}
pub struct Solution {}

struct CustomStack {
    stack: [i32; 1000],
    max_size: usize,
    size: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        return CustomStack {
            stack: [0; 1000],
            max_size: max_size as usize,
            size: 0,
        };
    }

    fn push(&mut self, x: i32) {
        if self.size < self.max_size {
            self.size += 1;
            self.stack[self.size - 1] = x;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.size == 0 {
            return -1;
        }

        let top = self.stack[self.size - 1];
        self.stack[self.size - 1] = 0;
        self.size -= 1;
        return top;
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..std::cmp::min(self.size, k as usize) {
            self.stack[i] += val;
        }
    }
}

fn main() {
    let mut stk = CustomStack::new(3);
    stk.push(1); // stack becomes [1]
    stk.push(2); // stack becomes [1, 2]
    stk.pop(); // return 2 --> Return top of the stack 2, stack becomes [1]
    stk.push(2); // stack becomes [1, 2]
    stk.push(3); // stack becomes [1, 2, 3]
    stk.push(4); // stack still [1, 2, 3], Do not add another elements as size is 4
    stk.increment(5, 100); // stack becomes [101, 102, 103]
    stk.increment(2, 100); // stack becomes [201, 202, 103]
    stk.pop(); // return 103 --> Return top of the stack 103, stack becomes [201, 202]
    stk.pop(); // return 202 --> Return top of the stack 202, stack becomes [201]
    stk.pop(); // return 201 --> Return top of the stack 201, stack becomes []
    stk.pop(); // return -1 --> Stack is empty return -1.
}

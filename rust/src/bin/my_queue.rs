// https://leetcode.com/problems/implement-queue-using-stacks/
// 2024/01/29

struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }

        return self.stack_out.pop().unwrap_or(0);
    }

    fn peek(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }

        return *self.stack_out.last().unwrap_or(&0);
    }

    fn empty(&self) -> bool {
        return self.stack_in.is_empty() && self.stack_out.is_empty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    let mut my_queue: MyQueue = MyQueue::new();
    my_queue.push(1); // queue is: [1]
    my_queue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
    println!("{:?}", my_queue.peek()); // return 1
    println!("{:?}", my_queue.pop()); // return 1, queue is [2]
    println!("{:?}", my_queue.empty()); // return false
}

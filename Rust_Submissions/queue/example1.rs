/*
Example: 933. Number of Recent Calls

Implement the RecentCounter class. It should support ping(int t), which records a call at time t, and then 
returns an integer representing the number of calls that have happened in the range [t - 3000, t]. Calls to ping 
will have increasing t.

*/
use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>
}


impl RecentCounter {

    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while !self.queue.is_empty() && *self.queue.front().unwrap() < t - 3000 {
            self.queue.pop_front();
        }
        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

fn main() {
    let mut counter = RecentCounter::new();
    println!("{}", counter.ping(1));
    println!("{}", counter.ping(100));
    println!("{}", counter.ping(3001));
    println!("{}", counter.ping(4000));
}



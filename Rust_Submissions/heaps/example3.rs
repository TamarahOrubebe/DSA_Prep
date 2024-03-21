
/*Example 3: 295. Find Median from Data Stream

The median is the middle value in an ordered integer list. If the size of the list is even, the median is the average of the two middle values.
Implement the MedianFinder class:

MedianFinder() initializes the MedianFinder object.

void addNum(int num) adds the integer num to the data structure.

double findMedian() returns the median of all elements so far.
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

impl MedianFinder {

    fn new() -> Self {
        Self{
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        if let Some(num) = self.max_heap.pop() {
            self.min_heap.push(Reverse(num));
            if self.min_heap.len() > self.max_heap.len() {
                if let Some(Reverse(num)) = self.min_heap.pop() {
                    self.max_heap.push(num);
                }
            }
        }
    }
    
    fn double_find_median(&mut self) -> i32 {
        if self.max_heap.len() > self.min_heap.len() {
            return *self.max_heap.peek().unwrap();
        }
        
        let min_num = (*self.min_heap.peek().unwrap()).0;
        let max_num = *self.max_heap.peek().unwrap();
        (min_num as f32 + max_num as f32 / 2.0) as i32
    }
}


fn main() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(4);
    median_finder.add_num(15);
    median_finder.add_num(6);
    median_finder.add_num(3);
    median_finder.add_num(8);
    median_finder.add_num(5);
    median_finder.add_num(7);
    median_finder.add_num(9);
    median_finder.add_num(11);
    median_finder.add_num(2);
    median_finder.add_num(12);
    median_finder.add_num(14);
    
   
    println!("The median is {}", median_finder.double_find_median());
}
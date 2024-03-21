// Example 1: 1046. Last Stone Weight

/*You are given an array of integers stones where stones[i] is the weight of the i^th stone. On each turn, we 
choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y 
with x <= y. If x == y, then both stones are destroyed. If x != y, then x is destroyed and y loses x weight. 
Return the weight of the last remaining stone, or 0 if there are no stones left.
*/
use std::collections::BinaryHeap;
fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        
        while heap.len() > 1 as usize {
            if let Some(first) = heap.pop() {
                if let Some(second) = heap.pop() {
                    if first != second {
                        heap.push(first - second);
                    }
                }
            }
        }
      let ans = if heap.len() > 0 {
          *heap.peek().unwrap()
      } else {
          0
      };
      ans
}

fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    println!("{:#?}", last_stone_weight(stones));
}

/*
On each smash, at least one rock is destroyed, so there can be at most n iterations. At each iteration, we 
perform pops and pushes on the heap, which has a length of n at the start. This gives us a time complexity of 
O(n⋅logn). The heap uses O(n) space. Note that in Python we are re-using the input, so we should count it 
towards the space complexity, which we wouldn't normally do.
*/
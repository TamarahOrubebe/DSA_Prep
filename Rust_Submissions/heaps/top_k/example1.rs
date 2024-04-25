// Example 1: 347. Top K Frequent Elements
// Given an integer array nums and an integer k, return the k most frequent elements. It is guaranteed that the 
// answer is unique.
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn top_k_elements(elements: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        for k in elements {
           *hash.entry(k).or_insert(0) += 1;
        }
        
        for (key, value) in hash {
            heap.push(Reverse((value, key)));
            if heap.len() > k as usize{
                heap.pop();
            }
        }
        
        heap.into_iter().map(|Reverse((_, elem))| elem).collect()
}    

fn main() {
    let elements = vec![1, 1, 2, 2, 3];
    println!("{:#?}", top_k_elements(elements, 2));
}

/*
Given nn as the length of nums, the time complexity of this algorithm is O(n.logk). 
Counting the frequencies only costs O(n), which will be dominated. In the main loop, we iterate nn times and 
perform some heap operations. The size of the heap never exceeds kk, so each iteration costs O(logk). The space 
complexity is O(k + n) for the heap and hashmap.
*/
    

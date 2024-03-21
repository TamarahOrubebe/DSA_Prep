// Example 2: 658. Find K Closest Elements
//Given a sorted integer array arr, two integers k and x, return the k closest integers to x. The answer should 
// also be sorted in ascending order. If there are ties, take the smaller elements.
use std::collections::BinaryHeap;

fn top_k_closest_elements(elements: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        
        for num in elements {
            let diff = (num - x).abs();
          heap.push((diff, num));
          if heap.len() > k as usize {
              heap.pop();
          }
        }

       let mut ans: Vec<i32> = heap.into_iter().map(|(_, elem)|elem).collect();
       ans.sort();
       ans
}

fn main() {
    let elements = vec![1, 4, 10, 15, 22];
    println!("{:#?}", top_k_closest_elements(elements, 3, 11));
}

/*
The time complexity of this algorithm is O((n+k)⋅logk), where n is the length of the input array. The heap size 
never grows beyond k, so the push and pop operations are O(logk), which we perform nn times. Then, we need 
O(k⋅logk) at the end to sort the output. This algorithm is actually pretty slow compared to other approaches 
since it does not exploit the fact that the input array is sorted - but we're using it as an example of how 
heaps can be used in "top k" problems. This approach is still faster than sorting with a custom comparator, 
which is the trivial approach.
*/
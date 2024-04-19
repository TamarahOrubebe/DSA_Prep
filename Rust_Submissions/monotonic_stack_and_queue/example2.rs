/*
Example 2: 239. Sliding Window Maximum

Given an integer array nums and an integer k, there is a sliding window of size k that moves from the very left 
to the very right. For each window, find the maximum element in the window.

For example, given nums = [1, 3, -1, -3, 5, 3, 6, 7], k = 3, return [3, 3, 5, 5, 6, 7]. The first window is 
[1, 3, -1, -3, 5, 3, 6, 7] and the last window is [1, 3, -1, -3, 5, 3, 6, 7]

*/
use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k:i32) -> Vec<i32> {
   let mut stack: VecDeque<usize> = VecDeque::new();
   let mut ans: Vec<i32> = Vec::new();
   
   for (i, &num) in nums.iter().enumerate() {
       while !stack.is_empty() && nums[*stack.back().unwrap()] < num {
           stack.pop_back();
       }
       
       stack.push_back(i);
       
       // check if window as moved past curr max num
       if *stack.front().unwrap() + k as usize == i {
           stack.pop_front();
       }
       
       // put max num in ans when window size is the same as the limit
       if i as i32 >= k - 1 {
           ans.push(nums[*stack.front().unwrap()]);
       }
   }
   
   ans
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    println!("The maximum element in each sliding window of length k is: {:#?}", max_sliding_window(nums, k)); 
    // return [3, 3, 5, 5, 6, 7]
}

/* 
This problem is quite difficult - try your best to understand the solution as it is a good demonstration of how 
powerful a deque is - the time complexity is O(n)O(n), where nn is the size of nums! The space complexity is O(k),
since the deque can't grow beyond that size. To summarize:

We use a monotonic decreasing deque, which implies that the first element is the maximum.
Once the maximum element is too far to stay in the window we remove it from the deque, and the next greatest 
element moves to position 0.
To maintain the decreasing order, we remove elements from the deque that are smaller than the elements being 
added.
*/


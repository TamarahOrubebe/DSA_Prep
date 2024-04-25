/*
Kth Largest Element in an Array

Solution
Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

Can you solve it without sorting?

 

Example 1:

Input: nums = [3,2,1,5,6,4], k = 2
Output: 5
Example 2:

Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
Output: 4
 

Constraints:

1 <= k <= nums.length <= 105
-104 <= nums[i] <= 104

*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        
        for num in nums {
            min_heap.push(Reverse(num));
            
            while min_heap.len() as i32 > k {
                min_heap.pop();
            }
        }
        
        (*min_heap.peek().unwrap()).0
}

// time complexity of O(n.log k) space complexity of O(k)
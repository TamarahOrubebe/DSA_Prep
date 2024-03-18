/*
Jump Game III

Solution
Given an array of non-negative integers arr, you are initially positioned at start index of the array. When you 
are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach any index with value 0.

Notice that you can not jump outside of the array at any time.

 

Example 1:

Input: arr = [4,2,3,0,3,1,2], start = 5
Output: true
Explanation: 
All possible ways to reach at index 3 with value 0 are: 
index 5 -> index 4 -> index 1 -> index 3 
index 5 -> index 6 -> index 4 -> index 1 -> index 3 
Example 2:

Input: arr = [4,2,3,0,3,1,2], start = 0
Output: true 
Explanation: 
One possible way to reach at index 3 with value 0 is: 
index 0 -> index 4 -> index 1 -> index 3
Example 3:

Input: arr = [3,0,2,1,2], start = 2
Output: false
Explanation: There is no way to reach at index 1 with value 0.
 

Constraints:

1 <= arr.length <= 5 * 104
0 <= arr[i] < arr.length
0 <= start < arr.length
*/

use std::collections::VecDeque;
use std::collections::HashSet;

fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let length = arr.len();
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut seen: Vec<bool> = vec![false; length];
        queue.push_back(start);
        while !queue.is_empty() {
            let curr_len = queue.len();
            
            for _ in 0..curr_len {
                if let Some(node) = queue.pop_front() {
                    if arr[node as usize] == 0 {
                        return true;
                    }
                    seen[node as usize] = true;
                    for j in vec![node + arr[node as usize], node - arr[node as usize]] {
                        if 0 <= j && j < length as i32 && !seen[j as usize] {
                            seen[j as usize] = true;
                            queue.push_back(j);
                        }
                    }
                   
                }
            }
        }
        false
}

/*
 Contiguous Array

Solution
Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.

 

Example 1:

Input: nums = [0,1]
Output: 2
Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
Example 2:

Input: nums = [0,1,0]
Output: 2
Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
 

Constraints:

1 <= nums.length <= 105
nums[i] is either 0 or 1.
*/

use std::collections::HashMap;

fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut ans = 0;
    let mut hash = HashMap::new();
    hash.insert(0, -1);

    for (right, num) in nums.iter().enumerate() {
        count += if *num == 1 { 1 } else { -1 };

        if let Some(&prev_index) = hash.get(&count) {
            ans = ans.max(right as i32 - prev_index);
        } else {
            hash.insert(count, right as i32);
        }
    }

    ans
}
fn main() {
    let nums = vec![0, 1, 0, 0, 1, 1];
    println!("{}", find_max_length(nums)); // Output: 6
}
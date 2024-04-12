/*
Max Consecutive Ones III

Solution
Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.

 

Example 1:

Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
Output: 6
Explanation: [1,1,1,0,0,1,1,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
Example 2:

Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
Output: 10
Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
 

Constraints:

1 <= nums.length <= 105
nums[i] is either 0 or 1.
0 <= k <= nums.length
*/


pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut curr = 0;
    let mut ans = 0;
        
    for right in 0..nums.len() {
        if nums[right] == 0 {
            curr += 1;
        }
            
        while curr > k {
            if nums[left] == 0 {
                curr -= 1;
            }
                
                left += 1;
        }
            
        ans = ans.max((right - left + 1) as i32);
    }
    ans
}

// the work done in each loop iteration is amortized constant, so this algorithm has a runtime of O(n),
// where n is the length of nums, and O(1) space.
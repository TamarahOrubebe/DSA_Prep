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

/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxLength = function(nums) {
    let count = ans = 0;
    let hash = new Map();
    hash.set(0, -1);
    for(let right = 0; right < nums.length; right++) {
        count += nums[right] == 1? 1 : -1
        
        if(hash.has(count)) {
            ans = Math.max(ans, right - hash.get(count));
        } else {
            hash.set(count, right);
        }
    }
    
   
    return ans;
};
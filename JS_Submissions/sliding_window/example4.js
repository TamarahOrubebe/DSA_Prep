/*
Given an integer array nums and an integer k, find the sum of the subarray with the largest sum whose length is k.

*/

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
let findBestSubarray = function(nums, k) {
    let curr = 0;
    for (let i = 0; i < k; i++) {
        curr += nums[i];
    }
    
    let ans = curr;
    for (let i = k; i < nums.length; i++) {
        curr += nums[i] - nums[i - k];
        ans = Math.max(ans, curr);
    }
    
    return ans;
}


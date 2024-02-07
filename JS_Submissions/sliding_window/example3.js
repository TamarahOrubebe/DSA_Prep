/*
Given an array of positive integers nums and an integer k, return the number of subarrays where the product of 
all the elements in the subarray is strictly less than k.

For example, given the input nums = [10, 5, 2, 6], k = 100, the answer is 8. The subarrays with products less 
than k are:

[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
*/

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
let numSubarrayProductLessThanK = function(nums, k) {
    if (k <= 1) {
        return 0;
    }

    let ans = 0, left = 0, curr = 1;
    
    for (let right = 0; right < nums.length; right++) {
        curr *= nums[right];
        while (curr >= k) {
            curr /= nums[left];
            left++;
        }
        
        ans += right - left + 1;
    }
    
    return ans;
};

// the work done in each loop iteration is amortized constant, so this algorithm has a runtime of O(n), 
// where nn is the length of nums, and O(1) space.
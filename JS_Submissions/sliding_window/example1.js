/* 
Given an array of positive integers nums and an integer k, find the length of the longest subarray whose sum is 
less than or equal to k. This is the problem we have been talking about above. We will now formally solve it.

Let's use an integer curr that tracks the sum of the current window. Since the problem wants subarrays whose sum 
is less than or equal to k, we want to maintain curr <= k. 
Let's look at an example where nums = [3, 1, 2, 7, 4, 2, 1, 1, 5] and k = 8.

*/


/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
let findLength = function(nums, k) {
    // curr is the current sum of the window
    let left = 0, curr = 0, ans = 0;
    for (let right = 0; right < nums.length; right++) {
        curr += nums[right];
        while (curr > k) {
            curr -= nums[left];
            left++;
        }
        
        ans = Math.max(ans, right - left + 1);
    }
    
    return ans;
}

/*
 this algorithm has a time complexity of O(n) since all work done inside the for loop is amortized O(1), 
 where nn is the length of nums. The space complexity is constant because we are only using 3 integer variables.
 
 */
/* 
Example 2: 300. Longest Increasing Subsequence

Given an integer array nums, return the length of the longest strictly increasing subsequence.

Let's say we're at index 7 and the current value is 5. We're interested in forming a strictly increasing 
subsequence where this 5 is the final value.

Because the 5 needs to be the final value, let's check the values before the current index, at indices 0 to 6.

We find that nums[5] = 4, which of course is less than the current value. We also find that dp[5] = 3. By 
definition of dp, that means there is an increasing subsequence of length 3 that ends with nums[5].

Because nums[7] > nums[5], we can take that subsequence and just append nums[7] to it, forming an increasing 
subsequence of length 4. Now, we don't need to know what the subsequence ending at nums[5] is, but we don't care
- we are only concerned about the length.

To calculate dp[7], we need to check all the indices from 0 to 6.

How did we know the value of dp[5]? Because it was a smaller subproblem. We start by solving dp[0] 
(which must be the base case 1), then dp[1] (which we can calculate from dp[0]), and so on.
*/

/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
    let dp = i => {
        if (memo[i] != -1) {
            return memo[i];
        }
        
        let ans = 1; // Base case
        for (let j = 0; j < i; j++) {
            if (nums[i] > nums[j]) {
                ans = Math.max(ans, dp(j) + 1);
            }
        }
        
        memo[i] = ans;
        return memo[i];
    }
    
    let memo = new Array(nums.length).fill(-1);
    let ans = 0;
    for (let i = 0; i < nums.length; i++) {
        ans = Math.max(ans, dp(i));
    }
    
    return ans;
};


/* 
BOTTOM UP APPROACH

var lengthOfLIS = function(nums) {
    let dp = new Array(nums.length).fill(1);
    for (let i = 0; i < nums.length; i++) {
        for (let j = 0; j < i; j++) {
            if (nums[i] > nums[j]) {
                dp[i] = Math.max(dp[i], dp[j] + 1);
            }
        }
    }
    
    return Math.max(...dp);
};

*/

// Because of the nested loop, this algorithm has a time complexity of O(n^2), 
// where nn is the length of the input array.Notice that this is because the work done at each state is linear
// with n, and there are n states.The space complexity is equal to the number of states, O(n), and can't be 
// improved in bottom - up because the recurrence relation is not static.
/*
Example 2: 2294. Partition Array Such That Maximum Difference Is K

Given an integer array nums and an integer k, split nums into subsequences, where each subsequences' maximum and
minimum element is within k of each other. What is the minimum number of subsequences needed?

For example, given nums = [3, 6, 1, 2, 5] and k = 2, the answer is 2. The subsequences are [3, 1, 2] and [6, 5]
*/

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var partitionArray = function(nums, k) {
    nums.sort((a, b) => a - b);
    let ans = 1;
    let x = nums[0];
    
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] - x > k) {
            x = nums[i];
            ans++;
        }
    }
    
    return ans;
};

/* 
To summarize: set x at the start and take as many elements as you can. Once you go beyond x + k, increment the 
answer and start again with a new x. This runs in O(n \cdot \log{}n)O(n⋅logn) where nn is the length of the 
input array due to the sort. Again, the only extra space used is during the sort, and the complexity depends on 
the sorting algorithm used by your language of choice.
*/
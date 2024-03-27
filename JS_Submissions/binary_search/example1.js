/*
Example 1: 704. Binary Search

You are given an array of integers nums which is sorted in ascending order, and an integer target. If target 
exists in nums, return its index. Otherwise, return -1.

We'll start with a simple example to show the code implementation. The brute force solution would be to perform 
a linear scan - just check every index. 


*/

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
    let left = 0;
    let right = nums.length - 1;
    while (left <= right) {
        let mid = Math.floor((left + right) / 2);
        let num = nums[mid];
        if (num == target) {
            return mid;
        }
        
        if (num > target) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return -1;
};

// This runs in O(n), where nn is the length of the input array. Because the array is sorted, we can use binary
// search to improve the time complexity to O(logn).

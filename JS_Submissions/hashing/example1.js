/*
Checking for existence
Report Issue
One of the most common applications of a hash table or set is determining if an element exists in O(1)O(1). 
Since an array needs O(n)O(n) to do this, using a hash map or set can improve the time complexity of an 
algorithm greatly, usually from O(n^2) to O(n). Let's look at some example problems.

Example 1: 1. Two Sum

Given an array of integers nums and an integer target, return indices of two numbers such that they add up to 
target. You cannot use the same index twice.

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    let dic = new Map();
    for (let i = 0; i < nums.length; i++) {
        let num = nums[i];
        let complement = target - num;
        if (dic.has(complement)) {
            return [i, dic.get(complement)];
        }
        
        dic.set(num, i);
    }

    return [-1, -1];
};

/*
If the question wanted us to return a boolean indicating if a pair exists or to return the numbers themselves, 
then we could just use a set. However, since it wants the indices of the numbers, we need to use a hash map to 
"remember" what indices the numbers are at.

The time complexity is O(n)O(n) as the hash map operations are O(1)O(1). This solution also uses O(n)O(n) space 
as the number of keys the hash map will store scales linearly with the input size.

*/
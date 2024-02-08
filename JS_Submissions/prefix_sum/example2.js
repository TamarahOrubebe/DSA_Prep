/* 
Given an integer array nums, find the number of ways to split the array into two parts so that the first section
has a sum greater than or equal to the sum of the second section. The second section should have at least one 
number.

*/


// If we build a prefix sum first, then iterate over each index, we can calculate the sums of the left and right
// sections in O(1), which would improve the time complexity to O(n).
/**
 * @param {number[]} nums
 * @return {number}
 */
var waysToSplitArray = function(nums) {
    let n = nums.length;
    
    let prefix = [nums[0]];
    for (let i = 1; i < n; i++) {
        prefix.push(nums[i] + prefix[prefix.length - 1]);
    }
    
    let ans = 0;
    for (let i = 0; i < n - 1; i++) {
        let leftSection = prefix[i];
        let rightSection = prefix[n - 1] - prefix[i];
        if (leftSection >= rightSection) {
            ans++;
        }
    }
    
    return ans;
};


// We actually do not need the array. We can calculate the left section on the fly as we move. To calculate the
// right section we can pre compute the sum of the nums array and subtract the left section from that.

/**
 * @param {number[]} nums
 * @return {number}
 */
var waysToSplitArray = function(nums) {
    let ans = 0, leftSection = 0, total = 0;
    for (const num of nums) {
        total += num;
    }
    
    for (let i = 0; i < nums.length - 1; i++) {
        leftSection += nums[i];
        let rightSection = total - leftSection;
        if (leftSection >= rightSection) {
            ans++;
        }
    }
    
    return ans;
};

// We have improved the space complexity to O(1)O(1), which is a great improvement.
/* 
Given a sorted array of unique integers and a target integer, return true if there exists a pair of numbers that
sum to target, false otherwise. This problem is similar to Two Sum. (In Two Sum, the input is not sorted).

For example, given nums = [1, 2, 4, 6, 8, 9, 14, 15] and target = 13, return true because 4 + 9 = 13.
*/

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {boolean}
 */
let checkForTarget = function(nums, target) {
    let left = 0;
    let right = nums.length - 1;
    
    while (left < right) {
        // curr is the current sum
        let curr = nums[left] + nums[right];
        if (curr == target) {
            return true;
        }
        
        if (curr > target) {
            right--;
        } else {
            left++;
        }
    }
    
    return false;
}

// this algorithm uses O(1) space and has a time complexity of O(n).
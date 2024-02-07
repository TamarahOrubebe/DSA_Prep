/*
Squares of a Sorted Array

Solution
Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

 

Example 1:

Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].
Example 2:

Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]
 

Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums is sorted in non-decreasing order.

*/

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut first: usize = 0;
    let length = nums.len();
    let mut second: usize = length - 1;
    let mut result = vec![0; length];
        
        
    for i in (0..=length - 1).rev() {
        if i32::abs(nums[first]) < i32::abs(nums[second]) {
           result[i] = i32::pow(nums[second], 2);
           second -= 1;

        } else {
            result[i] = i32::pow(nums[first], 2);
            first += 1;
            
        }
    }
    result
}

// this algorithm has a time complexity of O(n) and a space complexity of O(1).
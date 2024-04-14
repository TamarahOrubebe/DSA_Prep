/*
Example 3: Given an integer array nums, find all the unique numbers x in nums that satisfy the following: x + 1 
is not in nums, and x - 1 is not in nums.
*/

use std::collections::HashSet;

fn find_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let nums_set: HashSet<_> = nums.iter().cloned().collect();

    for &num in &nums_set {
        if !nums_set.contains(&(num + 1)) && !nums_set.contains(&(num - 1)) {
            ans.push(num);
        }
    }

    ans
}


/*
Because the checks are O(1), the time complexity is O(n) since each for loop iteration runs in constant time. 
The set will occupy O(n) space.

Anytime you find your algorithm running if ... in ..., then consider using a hash map or set to store elements 
to have these operations run in O(1). 
*/
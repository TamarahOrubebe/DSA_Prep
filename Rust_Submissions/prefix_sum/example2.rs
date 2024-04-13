/*
Example 2: 2270. Number of Ways to Split Array

Given an integer array nums, find the number of ways to split the array into two parts so that the first section 
has a sum greater than or equal to the sum of the second section. The second section should have at least one 
number.
*/


/*
A brute force approach would be to iterate over each index i from 0 until nums.length - 1. For each index, 
iterate from 0 to i to find the sum of the left section, and then iterate from i + 1 until the end of the array 
to find the sum of the right section. This algorithm would have a time complexity of O(n^2)

If we build a prefix sum first, then iterate over each index, we can calculate the sums of the left and right 
sections in O(1), which would improve the time complexity to O(n).
*/
pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut prefix = vec![nums[0]];
    for i in 1..nums.len() {
        prefix.push(nums[i] + prefix[prefix.len() - 1]);
    }
    let n = nums.len();
    let mut ans = 0;
    for i in 0..nums.len() {
        let left_section = prefix[i];
        let right_section = prefix[n - 1] - prefix[i];
        if left_section >= right_section {
            ans += 1;
        }
    }
    
    ans
}

/*
Do we need the array?
In this problem, the order in which we need to access prefix is incremental: to find leftSection, we do prefix[i]
as i increments by 1 each iteration.

As such, to calculate leftSection we don't actually need the array. We can just initialize leftSection = 0 and 
then calculate it on the fly by adding the current element to it at each iteration.

What about rightSection? By definition, the right section contains all the numbers in the array that aren't in 
the left section. Therefore, we can pre-compute the sum of the entire input as total, then calculate rightSection
as total - leftSection.

We are still using the concept of a prefix sum as each value of leftSection represents the sum of a prefix. We 
have simply replicated the functionality using an integer instead of an array.
*/

pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut left_section, mut right_section) = (0, 0);
    let total: i32 = nums.iter().sum();
    

    let mut ans = 0;
    for i in 0..nums.len() {
        left_section += nums[i];
        right_section = total - left_section;
        if left_section >= right_section {
            ans += 1;
        }
    }
    
    ans
}

// We have improved the space complexity to O(1), which is a great improvement.

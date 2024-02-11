/*

Example 3: Given an integer array nums, find all the unique numbers x in nums that satisfy the following: 
x + 1 is not in nums, and x - 1 is not in nums.

We can solve this in a straightforward manner - just iterate through nums and check if x + 1 or x - 1 is in 
nums. By converting nums into a set beforehand, these checks will cost O(1).

Converting the input into a set beforehand is another example of pre-processing.

*/

let findNumbers = nums => {
    let ans = [];
    let numsSet = new Set(nums);

    for (const num of nums) {
        if (!numsSet.has(num + 1) && !numsSet.has(num - 1)) {
            ans.push(num);
        }
    }

    return ans;
}

/*
Because the checks are O(1)O(1), the time complexity is O(n)O(n) since each for loop iteration runs in constant 
time. The set will occupy O(n)O(n) space.

Anytime you find your algorithm running if ... in ..., then consider using a hash map or set to store elements 
to have these operations run in O(1)O(1). Try these upcoming practice problems with what was learned here.

*/
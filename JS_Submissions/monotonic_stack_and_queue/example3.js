/*
Example 3: 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

Given an array of integers nums and an integer limit, return the size of the longest subarray such that the 
absolute difference between any two elements of this subarray is less than or equal to limit.

The "maximum absolute difference between any two elements" is the maximum element minus the minimum element. 
The question is asking for the longest subarray and has a constraint on the subarray - max - min <= limit. We 
learned that these characteristics suggest using a sliding window.

We also just learned how to keep the maximum element in a sliding window in the previous example. We need to do 
that again here but also keep the minimum element.

Use two deques - one monotonic increasing and one monotonic decreasing. The monotonic increasing one has the 
minimum element in the window at the first index. The monotonic decreasing one has the maximum element in the 
window at the first index. Then, we can use the same sliding window format we learned in the arrays and strings 
chapter. Add elements to the deques from the right, remove them from the left when the max - min > limit, and 
make sure to maintain the deques at each iteration.

Recall that the formula for the length of a window is right - left + 1.


We can identify this as a sliding window problem. The constraint metric is the maximum element in the window 
minus the minimum element in the window. The numeric restriction is <= limit.

In the previous example we kept a monotonic decreasing data structure that only contained elements in the 
current window. This meant that the maximum element in the window was always the first element in the data 
structure.

If we instead had a monotonic increasing data structure, then the minimum element in the window will always 
be the first element in the data structure.

We also learned in the previous example that when the data structures are monotonic, then we can easily find 
the next maximum element when the maximum was removed from the window. The exact same logic applies for the 
minimum.

Now that we know how to maintain the maximum and minimum elements in a window, we can just apply the standard 
sliding window algorithm: add elements from the right, remove them from the left whenever the constraint is 
broken. As we add and remove elements, we need to maintain our monotonic data structures.
*/

/**
 * @param {number[]} nums
 * @param {number} limit
 * @return {number}
 */
var longestSubarray = function(nums, limit) {
    let increasing = [];
    let decreasing = [];
    let left = 0, ans = 0;
    
    for (let right = 0; right < nums.length; right++) {
        // maintain the monotonic deques
        while (increasing.length && increasing[increasing.length - 1] > nums[right]) {
            increasing.pop();
        }
        
        while (decreasing.length && decreasing[decreasing.length - 1] < nums[right]) {
            decreasing.pop();
        }
        
        increasing.push(nums[right]);
        decreasing.push(nums[right]);
        
        // maintain window property
        while (decreasing[0] - increasing[0] > limit) {
            if (nums[left] == decreasing[0]) {
                decreasing.shift();
            }
            if (nums[left] == increasing[0]) {
                increasing.shift();
            }
            left++;
        }
        
        ans = Math.max(ans, right - left + 1);
    }
    
    return ans;
};

// With efficient queues, this algorithm has a time and space complexity of O(n)O(n) as each for loop iteration 
// is amortized O(n) and the deques can grow to size nn, where nn is the size of nums.
/*
Example 1: 39. Combination Sum

Given an array of distinct positive integers candidates and a target integer target, return a list of all unique
combinations of candidates where the chosen numbers sum to target. The same number may be chosen from candidates
an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers
is different.

In this problem, we'll have an argument path represent the numbers in the current path (this is what curr did in 
the previous article). We'll have an argument start that represents where in the input we should start iterating
from (this is what i did in the previous article). Finally, we'll have curr represent the sum of all the numbers
in path. While this is not necessary because we could just calculate it from path, it makes our algorithm more 
efficient as we won't need to calculate it from scratch at every node.

When we add an element to path and move to a child node, we should also add the value to curr. When we move back
up the tree and remove an element from path, we should also subtract the value from curr. If adding an element 
would cause curr to exceed target, then we should not add it - the input doesn't have negative numbers, so if we
exceed target then we will never reach out.

Our base case is when curr = target. While the base cases will be leaf nodes, not all leaf nodes will be the 
base case. If we have a node where curr < target, but adding any number would cause curr > target, then that 
will also be a leaf. We should only add path to the answer if curr = target.



*/

/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
    let backtrack = (path, start, curr) => {
        if (curr == target) {
            ans.push([...path]);
            return;
        }
        
        for (let i = start; i < candidates.length; i++) {
            let num = candidates[i];
            if (curr + num <= target) {
                path.push(num);
                backtrack(path, i, curr + num);
                path.pop();
            }
        }
    }
    
    ans = [];
    backtrack([], 0, 0);
    return ans;
};

/*
e time complexity of this algorithm is approximately O(n^T/M), where n = candidates.length, T = target, and 
M = min(candidates). Recall that recursion in general can be thought of as a tree. The maximum depth of the 
tree in this problem is T/M - using the smallest number repeatedly until we exceed target. Each node in the tree
can have up to n children, which gives us O(n^T/M).

If not counting the output as extra space, the space used in this problem is for path, and the recursion call 
stack, both of which are O(T/M).
*/
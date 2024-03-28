/*
Example 1: 46. Permutations

Given an array nums of distinct integers, return all the possible permutations in any order.

For example, given nums = [1, 2, 3], return [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]].


First, to generate all permutations, we need to put every number in the first position. For each number in the 
first position, we need to try all other numbers in the second position, and so on.

We will model the backtracking with a tree. Every node in the tree represents a function call. For a given 
function call, the argument curr represents the current permutation we are buiding.

When we add an element to curr, we make another call to backtrack. This is equivalent to moving to a child. The 
root node represents an empty array, and the path from the root to any given node represents curr.

To generate/traverse this tree, we start by calling backtrack with curr = []. Then, we put the first number in 
the first position and call backtrack again. In the second call, we can't put the first number in the second 
position because we already used the first number, so we put the second number instead and call backtrack again.

Eventually, curr will have the same length as nums which indicates that we have used all numbers 
(since duplicates aren't allowed) and we have a valid permutation. This is a leaf node/base case - we add curr 
to the answer, and then return.

Every time we return, exactly like in DFS, we are moving back up the tree. Remember: the path from the root to a 
given node represents curr. When we return, we are removing the last node in the path. This means we also need 
to remove the last element from curr.

After we have tried all possibilities with the first number being in the first position, we try the second 
number in the first position and go through a subtree again. In general, in each call to backtrack, we iterate 
over the input and if we find a number is not in curr, we add it to curr and go through the subtree.
*/

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function(nums) {
    let backtrack = curr => {
        if (curr.length == nums.length) {
            ans.push([...curr]);
            return;
        }
        
        for (const num of nums) {
            if (!curr.includes(num)) {
                curr.push(num);
                backtrack(curr);
                curr.pop();
            }
        }
    }
    
    let ans = [];
    backtrack([]);
    return ans;
};

/*
The time complexity of this algorithm is very slow, but the input says that 1 <= nums.length <= 6, so it is to 
be expected. Let n = nums.length. The initial call to backtrack (the "root" of the tree), makes n calls. Each of 
those calls makes n - 1 calls (avoiding duplicates), and each of those makes n - 2, and so on. As such, we can 
expect approximately O(n!)O(n!) calls. In the function, we have a loop over the nn input elements, and in each 
iteration, we check if num is already in curr, which costs O(n)O(n). Thus, we can estimate each function call to
cost about O(n^2). Note that we could optimize this process by keeping track of elements in curr using a separate 
hash set, allowing us to perform the checks in O(1).

Now, using this logic, we could say that the time complexity is O(n^2 . n!), or O(n⋅n!) if we implemented the 
optimization. However, this is not the true time complexity 
 of the algorithm. The true time complexity is extremely complicated and requires higher-level mathematics. It 
 is typical for backtracking problems to have very difficult to calculate time complexities - but as long as you
present the logic like we have here, you will be fine in an interview.

*/
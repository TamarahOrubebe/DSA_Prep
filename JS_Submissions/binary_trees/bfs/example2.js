/*
Example 2: 515. Find Largest Value in Each Tree Row

Given the root of a binary tree, return an array of the largest value in each row of the tree.

As we saw in the previous example, at the start of each while loop iteration we have all nodes of the current 
level in the queue.

We could iterate over the queue before handling the nodes and find the maximum value.

Alternatively, we could find the maximum value while handling the nodes to be more efficient. We can initialize 
currMax at the start of the while loop iteration and then for each node, update currMax with node.val before 
pushing the children onto the queue. After the for loop, currMax will have the answer for the current level.

Because we initialize currMax at the start of each while loop iteration, we handle the levels separately.
*/

/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var largestValues = function(root) {
    if (!root) {
        return [];
    }
    
    let ans = [];
    let queue = [root];

    while (queue.length) {
        let currentLength = queue.length;
        let currMax = -Infinity; // this will store the largest value for the current level
        let nextQueue = [];

        for (let i = 0; i < currentLength; i++) {
            let node = queue[i];
            currMax = Math.max(currMax, node.val);
            if (node.left) {
                nextQueue.push(node.left);
            }
            if (node.right) {
                nextQueue.push(node.right);
            }
        }
        
        ans.push(currMax);
        queue = nextQueue;
    }
    
    return ans;
};

// The time and space complexity is O(n) for the same reasons as the previous example.
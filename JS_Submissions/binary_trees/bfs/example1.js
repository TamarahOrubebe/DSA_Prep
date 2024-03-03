/*
Example 1: 199. Binary Tree Right Side View

Given the root of a binary tree, imagine yourself standing on the right side of it. Return the values of the 
nodes you can see ordered from top to bottom.
*/

/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var rightSideView = function(root) {
    if (!root) {
        return [];
    }
    
    let ans = [];
    let queue = [root];

    while (queue.length) {
        let nodesInCurrentLevel = queue.length;
        let nextQueue = [];
        
        ans.push(queue[queue.length - 1].val); // this is the rightmost node for the current level
        for (let i = 0; i < nodesInCurrentLevel; i++) {
            let node = queue[i];
            if (node.left) {
                nextQueue.push(node.left);
            }
            if (node.right) {
                nextQueue.push(node.right);
            }
        }
        
        queue = nextQueue;
    }
    
    return ans;
};

// This algorithm has a time and space complexity of O(n)O(n) for the same reasons as the algorithms in the 
// previous article.We visit each node only once and perform a constant amount of work at each node.The queue 
// could hold up to O(n)O(n) nodes.


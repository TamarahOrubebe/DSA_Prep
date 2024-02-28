/*
Example 2: 112. Path Sum

Given the root of a binary tree and an integer targetSum, return true if there exists a path from the root to a 
leaf such that the sum of the nodes on the path is equal to targetSum, and return false otherwise.
*/

/**
 * @param {TreeNode} root
 * @param {number} targetSum
 * @return {boolean}
 */
var hasPathSum = function(root, targetSum) {
    let dfs = (node, curr) => {
        if (!node) {
            return false;
        }
        
        // if both children are null, then the node is a leaf
        if (!node.left && !node.right) {
            return (curr + node.val) == targetSum;
        }
        
        curr += node.val;
        let left = dfs(node.left, curr);
        let right = dfs(node.right, curr);
        return left || right;
    }
    
    return dfs(root, 0);
};

// time and space complexity O(n) 
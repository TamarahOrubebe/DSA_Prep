/*
Example 3: 1448. Count Good Nodes in Binary Tree

Given the root of a binary tree, find the number of nodes that are good. A node is good if the path between the 
root and the node has no nodes with a greater value.
*/

/**
 * @param {TreeNode} root
 * @return {number}
 */
var goodNodes = function(root) {
    let dfs = (node, maxSoFar) => {
        if (!node) {
            return 0;
        }
        
        let left = dfs(node.left, Math.max(maxSoFar, node.val));
        let right = dfs(node.right, Math.max(maxSoFar, node.val));
        let ans = left + right;
        if (node.val >= maxSoFar) {
            ans++;
        }
        
        return ans;
    }
    
    return dfs(root, -Infinity);
};

// time and space complexity O(n)
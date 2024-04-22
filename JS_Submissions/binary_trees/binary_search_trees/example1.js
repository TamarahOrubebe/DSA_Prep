/*
Example 1: 938. Range Sum of BST

Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes
with a value in the inclusive range [low, high].

The trivial approach would be to do a normal BFS or DFS, visit every node, and only add nodes whose values are 
between low and high to the sum. However, we can make use of the BST property to develop a more efficient 
algorithm. In a BST, every node has a value greater than all nodes in the left subtree and a value less than all 
nodes in the right subtree. Therefore, if the current node's value is less than low, we know it is pointless to 
check the left subtree because all nodes in the left subtree will be out of the range. Similarly, if the current 
node's value is greater than high, it is pointless to check the right subtree. This optimization can save a 
potentially huge amount of computation.
*/


/**
 * @param {TreeNode} root
 * @param {number} low
 * @param {number} high
 * @return {number}
 */
var rangeSumBST = function(root, low, high) {
    if (!root) {
        return 0;
    }
    
    let ans = 0;
    if (low <= root.val && root.val <= high) {
        ans += root.val;
    }
    if (low < root.val) {
        ans += rangeSumBST(root.left, low, high);
    }
    if (root.val < high) {
        ans += rangeSumBST(root.right, low, high);
    }
    
    return ans;
};

/* 
Although the time complexity is still O(n) for the case when all nodes in the tree are between low and high,
on average this algorithm will perform better than simply searching all nodes. For example, if you had a full 
tree with a million nodes, and the root's value was greater than high, then you can immediately save 500,000 
visits based on the logic that all nodes in the right subtree are greater than the root's value which is 
already outside the range.

The space complexity is O(n) for the stack / recursion call stack.

*/

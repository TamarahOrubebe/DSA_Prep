/* 
Example 2: 530. Minimum Absolute Difference in BST

Given the root of a BST, return the minimum absolute difference between the values of any two different nodes in 
the tree. 

One approach would be to go through the tree and put all the values in an array, then loop over all 
pairs of the array to find the minimum difference. This would be O(n^2)

A better approach would be to sort the array, and then iterate over the adjacent elements. The answer must be 
between adjacent elements in the sorted array, so this improves the time complexity to O(n⋅logn) due to the sort.
Can we do better?

As briefly mentioned earlier in this article, if you perform an inorder traversal on a BST, you will visit the 
nodes in sorted order. Therefore, if we do an inorder DFS, we can get the nodes in sorted order without the 
O(n⋅logn) sort, resulting in an overall time complexity of O(n).

We will pass an array values in our dfs function. To perform the inorder traversal, we first call on the left 
child, then add the current value to values, then call on the right child. This will add the values in sorted 
order.
*/

/**
 * @param {TreeNode} root
 * @return {number}
 */
var getMinimumDifference = function(root) {
    let dfs = node => {
        if (!node) {
            return;
        }
        
        dfs(node.left);
        values.push(node.val);
        dfs(node.right);
    }
    
    let values = [];
    dfs(root);
    let ans = Infinity;
    for (let i = 1; i < values.length; i++) {
        ans = Math.min(ans, values[i] - values[i - 1]);
    }
    
    return ans;
};

// The time and space complexity of this approach is O(n). We are able to get the values in sorted order in 
// linear time by taking advantage of the BST property.
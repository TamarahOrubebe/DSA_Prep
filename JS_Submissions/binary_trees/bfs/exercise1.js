/*
Deepest Leaves Sum

Solution
Given the root of a binary tree, return the sum of values of its deepest leaves.
 

Example 1:


Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
Output: 15
Example 2:

Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
Output: 19
 

Constraints:

The number of nodes in the tree is in the range [1, 104].
1 <= Node.val <= 100
*/

/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var deepestLeavesSum = function(root) {
    
    let queue = [root];
    let maxLevelSum = 0;
    
    while(queue.length) {
      
        let currLength = queue.length;
        let nextQueue = [];
        let currLevelSum = 0;
        
        for(let i = 0; i < currLength; i++) {
           let node = queue[i];
            currLevelSum += node.val;
            if(node.left) {
                nextQueue.push(node.left);
            }
            
            if(node.right) {
                nextQueue.push(node.right);
            }
        }
        maxLevelSum = currLevelSum;
       queue = nextQueue;
    }
    
   
    return maxLevelSum;
    
};

// time and space complexitie of O(n)
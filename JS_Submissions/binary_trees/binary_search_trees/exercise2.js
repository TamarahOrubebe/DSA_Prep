/*
Closest Binary Search Tree Value

Solution
Given the root of a binary search tree and a target value, return the value in the BST that is closest to the target. If there are multiple answers, print the smallest.

 

Example 1:


Input: root = [4,2,5,1,3], target = 3.714286
Output: 4
Example 2:

Input: root = [1], target = 4.428571
Output: 1
 

Constraints:

The number of nodes in the tree is in the range [1, 104].
0 <= Node.val <= 109
-109 <= target <= 109
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
 * @param {number} target
 * @return {number}
 */
var closestValue = function(root, target) {
    if(!root) return null;
    let closest = root.val;
   
    if(!root.left && !root.right) {
        return closest;
    }
    
    let minDiff = Math.abs(target - root.val);
    
    if(root.left) {
        let left = closestValue(root.left, target);  
        if(Math.abs(target - left) < minDiff) {
           closest = left;
        } else if(Math.abs(target - left) == minDiff) {
            closest = Math.min(closest, left);
        }
        
    }
   
    if(root.right) {
        let right = closestValue(root.right, target);
        if(Math.abs(target - right) < minDiff) {
            closest = right;
        } else if(Math.abs(target - right) == minDiff) {
            closest = Math.min(closest, right);
        }
    }
    
    return closest;
    
};

// time and space complexities O(n).
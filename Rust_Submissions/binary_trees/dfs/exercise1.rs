/*
Minimum Depth of Binary Tree

Solution
Given a binary tree, find its minimum depth.

The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

Note: A leaf is a node with no children.

 

Example 1:


Input: root = [3,9,20,null,null,15,7]
Output: 2
Example 2:

Input: root = [2,null,3,null,4,null,5,null,6]
Output: 5
 

Constraints:

The number of nodes in the tree is in the range [0, 105].
-1000 <= Node.val <= 1000
*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;


fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(inner) => {
                let mut inner_node = inner.borrow_mut();
                let left = Solution::min_depth(inner_node.left.take());
                let right = Solution::min_depth(inner_node.right.take());
                
                if left != 0 && right != 0 {
                   return left.min(right) + 1;
                }
                
                if left == 0 {
                    return right + 1;
                }
                
                left + 1
            },
            
            None => 0,
        }
}

// time and space complexities O(n);
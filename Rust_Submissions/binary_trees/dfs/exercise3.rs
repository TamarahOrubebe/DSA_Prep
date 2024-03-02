/*
Diameter of Binary Tree

Solution
Given the root of a binary tree, return the length of the diameter of the tree.

The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or
 may not pass through the root.

The length of a path between two nodes is represented by the number of edges between them.

 

Example 1:


Input: root = [1,2,3,4,5]
Output: 3
Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
Example 2:

Input: root = [1,2]
Output: 1
 

Constraints:

The number of nodes in the tree is in the range [1, 104].
-100 <= Node.val <= 100
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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        
            if let Some(inner) = node {
                let mut node_ref = inner.borrow_mut();
                let left = dfs(node_ref.left.take(), diameter);
                let right = dfs(node_ref.right.take(), diameter);
                
                *diameter = i32::max(*diameter, left + right);
                left.max(right) + 1 
            } else {
                0
            }
        };  
        
        let mut diameter: i32 = 0;
        dfs(root,&mut diameter);
        diameter
}    

// time and space complexities O(n);

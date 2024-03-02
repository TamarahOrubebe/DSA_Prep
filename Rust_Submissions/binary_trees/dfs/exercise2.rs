/*
 Maximum Difference Between Node and Ancestor

Solution
Given the root of a binary tree, find the maximum value v for which there exist different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.

A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.

 

Example 1:


Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
Output: 7
Explanation: We have various ancestor-node differences, some of which are given below :
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
Example 2:


Input: root = [1,null,2,null,0,3]
Output: 3
 

Constraints:

The number of nodes in the tree is in the range [2, 5000].
0 <= Node.val <= 105

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

fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut min: i32, mut max: i32) -> i32 {
            if let Some(inner_node) = node {
                let mut node_ref = inner_node.borrow_mut();

                // Update min and max values
                min = i32::min(min, node_ref.val);
                max = i32::max(max, node_ref.val);

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return max - min;
                }

                // Recursive calls for left and right children
                let left_diff = if let Some(left) = node_ref.left.take() {
                    dfs(&Some(left), min, max)
                } else {
                    0
                };
                let right_diff = if let Some(right) = node_ref.right.take() {
                    dfs(&Some(right), min, max)
                } else {
                    0
                };

                // Return the maximum difference found
                left_diff.max(right_diff)
            } else {
                0
            }
        }

        // Start the traversal from the root with initial min and max values
            let root_val = root.as_ref().unwrap().borrow().val;
            dfs(&root, root_val, root_val)
       
}

// time and space complexities O(n);
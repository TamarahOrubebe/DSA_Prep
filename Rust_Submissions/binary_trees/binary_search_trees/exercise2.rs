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
impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 { 

        if let Some(inner) = root {
            
            let mut node_ref = inner.borrow_mut();
            let mut closest = node_ref.val;
            let min_difference = (target - node_ref.val as f64).abs();
            
            if node_ref.left.is_none() && node_ref.right.is_none() {
                return closest;
            }
            
            if node_ref.left.is_some() {
                let left = Self::closest_value(node_ref.left.take(), target);
                if (target - left as f64).abs() < min_difference {
                    closest = left;
                } else if (target - left as f64).abs() == min_difference {
                    closest = closest.min(left);
                }
            }
            
            
            if node_ref.right.is_some() {
                    let right = Self::closest_value(node_ref.right.take(), target);
                    if (target - right as f64).abs() < min_difference {
                        closest = right;
                    } else if (target - right as f64).abs() == min_difference {
                        closest = closest.min(right);
                    }
            }
            closest
        } else {
            root.unwrap().borrow().val
        }
        
        
    }
}

// time and space complexities O(n).


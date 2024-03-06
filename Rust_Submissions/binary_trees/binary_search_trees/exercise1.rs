/* 
Insert into a Binary Search Tree

Solution
You are given the root node of a binary search tree (BST) and a value to insert into the tree. Return the root 
node of the BST after the insertion. It is guaranteed that the new value does not exist in the original BST.

Notice that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after 
insertion. You can return any of them.

 

Example 1:


Input: root = [4,2,7,1,3], val = 5
Output: [4,2,7,1,3,5]
Explanation: Another accepted tree is:

Example 2:

Input: root = [40,20,60,10,30,50,70], val = 25
Output: [40,20,60,10,30,50,70,null,null,25]
Example 3:

Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
Output: [4,2,7,1,3,5]
 

Constraints:

The number of nodes in the tree will be in the range [0, 104].
-108 <= Node.val <= 108
All the values Node.val are unique.
-108 <= val <= 108
It's guaranteed that val does not exist in the original BST.

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
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        
        if root.is_none() {
            root = node;
            return root;
        }
        
        if let Some(inner_node) = &root {
            let mut node_ref = inner_node.borrow_mut();
            
            if val < node_ref.val {
                if node_ref.left.is_some() {
                    Self::insert_into_bst(node_ref.left.clone(), val);
                } else {
                    node_ref.left = node;
                }
            } else {
                if node_ref.right.is_some() {
                    Self::insert_into_bst(node_ref.right.clone(), val);
                } else {
                    node_ref.right = node;
                }
            }
        }
        root
    }
}

// time and space complexities O(n).
/*
 Binary Tree Zigzag Level Order Traversal

Solution
Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left 
to right, then right to left for the next level and alternate between).

 

Example 1:


Input: root = [3,9,20,null,null,15,7]
Output: [[3],[20,9],[15,7]]
Example 2:

Input: root = [1]
Output: [[1]]
Example 3:

Input: root = []
Output: []
 

Constraints:

The number of nodes in the tree is in the range [0, 2000].
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
use::std::collections::VecDeque;

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut flip = true;
        
        while !queue.is_empty() {
            let curr_length = queue.len();
            let mut curr_level: VecDeque<i32> = VecDeque::new();

            for _ in 0..curr_length {
                if let Some(inner) = queue.pop_front().unwrap() {
                    let mut node_ref = inner.as_ref().borrow_mut();

                    if flip {
                        curr_level.push_back(node_ref.val);

                    } else {
                        curr_level.push_front(node_ref.val);
                    }
                    
                    if let Some(left) = node_ref.left.take() {
                            queue.push_back(Some(left));
                        }
                    if let Some(right) = node_ref.right.take() {
                            queue.push_back(Some(right));
                    }
                }
            }
            flip = !flip;
            ans.push(curr_level.into());
           
        }
        ans    
}    


// time and space complexities of O(n)
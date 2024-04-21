/*
Example 1: 199. Binary Tree Right Side View

Given the root of a binary tree, imagine yourself standing on the right side of it. Return the values of the 
nodes you can see ordered from top to bottom.
*/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
         if root.is_none() {
            return Vec::new();
        }
        let mut queue = VecDeque::new();
        let mut ans = Vec::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let curr_level_len = queue.len();
            let node_to_push = if let Some(ref node) = queue.back().unwrap() {
                let inner_node = node.borrow().val;
                inner_node
            } else {
                0
            };

            ans.push(node_to_push);
            for _ in 0..curr_level_len {
                if let Some(inner) = queue.pop_front().unwrap() {
                    let mut node_ref = inner.borrow_mut();
                    if node_ref.left.is_some() {
                        queue.push_back(node_ref.left.take())
                    }

                    if node_ref.right.is_some() {
                        queue.push_back(node_ref.right.take());
                    }
                }
            }
        }
        ans  
    }
    


fn main() {
    // Creating the binary tree from the array
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
    })));

    // Example usage of the right_side_view algorithm
    let result = right_side_view(root);
    println!("Right side view: {:?}", result); // Output: [1, 3, 4]
}


// time and space complexities of O(n);
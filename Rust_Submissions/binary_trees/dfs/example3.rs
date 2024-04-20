/*
Example 2: 112. Path Sum

Given the root of a binary tree and an integer targetSum, return true if there exists a path from the root to a 
leaf such that the sum of the nodes on the path is equal to targetSum, and return false otherwise.
*/

// Definition for a binary tree node.
//  #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

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
 pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_so_far: i32) -> i32 {
            if let Some(n) = node {
                let mut inner_node = n.borrow_mut();
                let mut ans = 0;
                if inner_node.val >= max_so_far {
                    ans += 1;
                }

                ans += dfs(inner_node.left.take(), max_so_far.max(inner_node.val));
                ans += dfs(inner_node.right.take(), max_so_far.max(inner_node.val));
                ans

            } else {
                0
            }
        }
        dfs(root, i32::MIN)
}

fn main() {
    // Creating the binary tree from the array
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));

    // Example usage of the good_nodes algorithm
    let result = good_nodes(root);
    println!("Number of good nodes: {}", result);
}

// time and space complexity O(n);
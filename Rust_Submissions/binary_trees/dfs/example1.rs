/*
Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the 
farthest leaf node.

 
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
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
     if let Some(node) = root {
        let mut inner_node = node.borrow_mut();

        let left = super::max_depth(inner_node.left.take());
        let right = super::max_depth(inner_node.right.take());
        left.max(right) + 1
     } else {
        0
     }
}

/*
The time and space complexity of tree questions is usually straightforward. The time complexity is almost always
O(n), where n is the total number of nodes, because each node is only visited once, and at each node, O(1) work 
is done. If more than O(1) work is done at each node, let's say O(k) work, then the time complexity will be O(n⋅k).

For space complexity, even if you are using recursion, the calls are still placed on the call stack which counts
as extra space. The largest the stack will be (for either iterative or recursive) at any time will depend on the
tree. For recursion, in the worst case it is O(n) if the tree is just a straight line, so usually, the correct 
answer to give for space complexity is O(n). If the tree is "complete" (all nodes have 0 or 2 children and each 
level except the last is full), then the space complexity is O(logn), but this is a best-case scenario.
*/
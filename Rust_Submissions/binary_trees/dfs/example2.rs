/*
Example 2: 112. Path Sum

Given the root of a binary tree and an integer targetSum, return true if there exists a path from the root to a 
leaf such that the sum of the nodes on the path is equal to targetSum, and return false otherwise.
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
fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
     fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut sum: i32, target_sum: i32) -> i32 {
        if let Some(n) = node {
            let mut inner_node = n.borrow_mut();
            if inner_node.left.is_none() && inner_node.right.is_none() {
                return (sum + inner_node.val) == target_sum;
            }

            sum += inner_node.val;
            let left = dfs(inner_node.left.take(), sum, target_sum);
            let right = dfs(inner_node.right.take(), sum, target_sum);
            left || right
        } else {
            false
        }
     }

     dfs(root, 0, target_sum)
}

fn main() {
    // Create the nodes
    let node_5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node_4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node_8 = Rc::new(RefCell::new(TreeNode::new(8)));
    let node_11 = Rc::new(RefCell::new(TreeNode::new(11)));
    let node_13 = Rc::new(RefCell::new(TreeNode::new(13)));
    let node_4_2 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_1 = Rc::new(RefCell::new(TreeNode::new(1)));

    // Build the tree structure
    node_5.borrow_mut().left = Some(node_4.clone());
    node_5.borrow_mut().right = Some(node_8.clone());

    node_4.borrow_mut().left = Some(node_11.clone());

    node_8.borrow_mut().left = Some(node_13.clone());
    node_8.borrow_mut().right = Some(node_4_2.clone());

    node_11.borrow_mut().left = Some(node_7.clone());
    node_11.borrow_mut().right = Some(node_2.clone());

    node_4_2.borrow_mut().right = Some(node_1.clone());

    // The root node
    let root = Some(node_5);
    
    println!("The answer for the algorithm is: {}", has_path_sum(root, 22));
}

// time and space complexity O(n) 
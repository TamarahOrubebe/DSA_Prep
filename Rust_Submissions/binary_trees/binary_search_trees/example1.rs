/*
Example 1: 938. Range Sum of BST

Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes
with a value in the inclusive range [low, high].

The trivial approach would be to do a normal BFS or DFS, visit every node, and only add nodes whose values are 
between low and high to the sum. However, we can make use of the BST property to develop a more efficient 
algorithm. In a BST, every node has a value greater than all nodes in the left subtree and a value less than all 
nodes in the right subtree. Therefore, if the current node's value is less than low, we know it is pointless to 
check the left subtree because all nodes in the left subtree will be out of the range. Similarly, if the current 
node's value is greater than high, it is pointless to check the right subtree. This optimization can save a 
potentially huge amount of computation.
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
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node) = root {
        let node_ref = node.borrow();
        let mut ans = 0;
        if low <= node_ref.val && node_ref.val <= high {
            ans += node_ref.val;
        }
        if low < node_ref.val {
            ans += range_sum_bst(node_ref.left.clone(), low, high);
        }
        if node_ref.val < high {
            ans += range_sum_bst(node_ref.right.clone(), low, high);
        }
        ans
    } else {
        0
    }
}

fn main() {
    // Construct the binary search tree
    let mut root = TreeNode::new(10);
    let mut left = TreeNode::new(5);
    let mut right = TreeNode::new(15);
    let left_left = TreeNode::new(3);
    let left_right = TreeNode::new(7);
    let right_right = TreeNode::new(18);

    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
   

    let low = 7;
    let high = 15;

    // Calculate the range sum
    let sum = range_sum_bst(Some(Rc::new(RefCell::new(root))), low, high);
    println!("Range sum between {} and {}: {}", low, high, sum);
}

/* 
Although the time complexity is still O(n) for the case when all nodes in the tree are between low and high,
on average this algorithm will perform better than simply searching all nodes. For example, if you had a full 
tree with a million nodes, and the root's value was greater than high, then you can immediately save 500,000 
visits based on the logic that all nodes in the right subtree are greater than the root's value which is 
already outside the range.

The space complexity is O(n) for the stack / recursion call stack.

*/
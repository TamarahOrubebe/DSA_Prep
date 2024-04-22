/* 
Example 2: 530. Minimum Absolute Difference in BST

Given the root of a BST, return the minimum absolute difference between the values of any two different nodes in 
the tree. 

One approach would be to go through the tree and put all the values in an array, then loop over all 
pairs of the array to find the minimum difference. This would be O(n^2)

A better approach would be to sort the array, and then iterate over the adjacent elements. The answer must be 
between adjacent elements in the sorted array, so this improves the time complexity to O(n⋅logn) due to the sort.
Can we do better?

As briefly mentioned earlier in this article, if you perform an inorder traversal on a BST, you will visit the 
nodes in sorted order. Therefore, if we do an inorder DFS, we can get the nodes in sorted order without the 
O(n⋅logn) sort, resulting in an overall time complexity of O(n).

We will pass an array values in our dfs function. To perform the inorder traversal, we first call on the left 
child, then add the current value to values, then call on the right child. This will add the values in sorted 
order.
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

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
    if let Some(n) = node {
        let n_ref = n.borrow_mut();
        dfs(n_ref.left.take(), values);
        values.push(n_ref.val);
        dfs(n_ref.right.take(), values);
    }
}

fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut values = Vec::new();
    dfs(root, &mut values);
    let mut min_diff = i32::MAX;
    for i in 1..values.len() {
        min_diff = min_diff.min(values[i] - values[i - 1]);
    }
    min_diff
}

fn main() {
    // Construct the binary search tree
    let mut root = TreeNode::new(4);
    let mut left = TreeNode::new(2);
    let mut right = TreeNode::new(6);
    let left_left = TreeNode::new(1);
    let left_right = TreeNode::new(3);
    let right_right = TreeNode::new(7);

    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    

    // Calculate the minimum difference
    let min_diff = get_minimum_difference(Some(Rc::new(RefCell::new(root))));
    println!("Minimum difference between tree nodes: {}", min_diff);
}

/*
The time and space complexity of this approach is O(n). We are able to get the values in sorted order in 
linear time by taking advantage of the BST property.

*/
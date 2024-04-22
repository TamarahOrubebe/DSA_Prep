/*
Example 3: 98. Validate Binary Search Tree

Given the root of a binary tree, determine if it is a valid BST.

Recall an earlier example problem where we needed to determine if two trees were the same tree. In that problem, 
we determined that if a node p and q represent the same tree, then their left and right subtrees must also be 
identical to each other.

If a tree rooted at node is a binary search tree, then node.left and node.right must also be binary search 
trees. Because the function isValidBST we are implementing determines if a tree is a binary search tree, we have 
a recursive way to look at the problem.

isValidBST(node.left) && isValidBST(node.right) must be true, and also the current node's value must not violate 
the BST property.

To determine if a node's value is violating the BST property, we can use two arguments small and large. These 
represent the (exclusive) range (small, large) in which a node's value should fall under. If a node's value is not in this range, then it is violating the BST property and we can return false.

The root node can have any value, so we initialize small = -infinity and large = infinity.

Every node in the root's left subtree must be less than root.val, so when we call on the left subtree, we can 
pass large = root.val.

Every node in the root's right subtree must be greater than root.val, so when we call on the right subtree, 
we can pass small = root.val.

By updating small and large in this manner, we ensure that the constraint that determines if a node's value is 
acceptable is always accurate, as recursion will keep a copy of small and large for each node.

As a base case, when we encounter an empty tree, we return true. Again, think about the case where the input 
tree is a single node. Any node on its own is by definition a binary search tree, so we would need both 
isValidBST(root.left) and isValidBST(root.right) to return true, so we need the empty tree to return true.

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

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, small: i64, large: i64) -> bool {
    if let Some(n) = node {
        let mut n_ref = n.borrow();
        if (small as i32) >= n_ref.val || n_ref.val >= (large as i32) {
            return false;
        }
        dfs(n_ref.left.take(), small, n_ref.val as i64) && 
               dfs(n_ref.right.take(), n_ref.val as i64, large)
    } else {
        true
    }
    
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(root, i64::MIN, i64::MAX)
}

fn main() {
    // Construct the binary search tree
    let mut root = TreeNode::new(2);
    let  left = TreeNode::new(1);
    let  right = TreeNode::new(3);

    
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    // Check if the tree is a valid binary search tree
    let is_valid = is_valid_bst(Some(Rc::new(RefCell::new(root))));
    println!("Is the tree a valid binary search tree? {}", is_valid);
}

// The time and space complexity are both O(n) for the same reasons as all the other examples.


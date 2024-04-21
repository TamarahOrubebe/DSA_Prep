/*
Example 2: 515. Find Largest Value in Each Tree Row

Given the root of a binary tree, return an array of the largest value in each row of the tree.
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

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let curr_level_len = queue.len();
            let mut curr_max = i32::MIN;
            for _ in 0..curr_level_len {
                if let Some(inner) = queue.pop_front().unwrap() {
                    let mut node_ref = inner.borrow_mut();
                    curr_max = curr_max.max(node_ref.val);
                    if node_ref.left.is_some() {
                        queue.push_back(node_ref.left.take())
                    }

                    if node_ref.right.is_some() {
                        queue.push_back(node_ref.right.take());
                    }
                }
            }
            ans.push(curr_max)
        }
        ans
    }
    


fn main() {
    // Creating the binary tree from the array
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        }))),
    })));
    // Example usage of the right_side_view algorithm
    let result = largest_values(root);
    println!("Right side view: {:?}", result); // Output: [1, 3, 9]
}

/*
With an efficient queue, the dequeue and enqueue operations are O(1)O(1), which means that the time complexity 
of BFS is the same as DFS. Again, the main idea is that we visit each node only once, so the time complexity is 
O(n⋅k) where nn is the total number of nodes, and kk is the amount of work we do at each node, usually O(1)
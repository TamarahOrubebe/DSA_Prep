/*
Just like DFS, the code/implementations for BFS is very similar across different problems. Here is a general 
format (we're printing the values of the nodes as an example):

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
use std::collections::VecDeque;

impl Solution {
    pub fn breadth_first_search(root: Option<Rc<RefCell<TreeNode>>>)  {
        if root.is_none() {
            return;
        }
        
        let mut queue = VecDeque::new();
        queue.push_back(root);
        
        while !queue.is_empty() {
            let curr_length = queue.len();
        
            
            for _ in 0..curr_length {
                
                if let Some(node) = queue.pop_front().unwrap() {
                    let mut node_ref = node.borrow_mut();
                    println!(node_ref.val);

                    if node_ref.left.is_some() {
                        queue.push_back(node_ref.left.take());
                    }

                    if node_ref.right.is_some() {
                        queue.push_back(node_ref.right.take());
                    }
                }
                
            }
        }
    }
}

Note for JavaScript users: JavaScript doesn't support a built-in efficient queue, but we can work around this 
by using a second array nextQueue to implement an efficient BFS.

With an efficient queue, the dequeue and enqueue operations are O(1), which means that the time complexity 
of BFS is the same as DFS. Again, the main idea is that we visit each node only once, so the time complexity is 
O(n⋅k) where nn is the total number of nodes, and kk is the amount of work we do at each node, 
usually O(1). Let's look at some example problems.
*/

fn main() {
    print!("Hello world!");
}
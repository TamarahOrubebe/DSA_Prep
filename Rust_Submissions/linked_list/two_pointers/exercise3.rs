/*
 Reverse Linked List II

Solution
Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.

 

Example 1:


Input: head = [1,2,3,4,5], left = 2, right = 4
Output: [1,4,3,2,5]
Example 2:

Input: head = [5], left = 1, right = 1
Output: [5]
 

Constraints:

The number of nodes in the list is n.
1 <= n <= 500
-500 <= Node.val <= 500
1 <= left <= right <= n
 

Follow up: Could you do it in one pass?
*/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        let left = left - 1;
        
        for _ in 0..left { // this takes you to the node just before the left node with curr pointing at that 
            //node's next node which is the left node itself.
            if let Some(node) = curr {
                curr = &mut node.next;
            } else {
                // if the left is out of bounds like greater than th length of the linked list return the head
                return head;
            }
        }
        
        // This takes ownership of the rest of the list i.e from the node at position left and hands it over to
        // the rest variable.
        let mut rest = curr.take();
        let mut prev = None;
        
        // This loop reverses the linked list from the left node to the right node.
        // at the end of the loop, rest points to the node after the right node.
        for _ in left..right {
            if let Some(mut node) = rest {
                rest = node.next;
                node.next = prev;
                prev = Some(node);
            }
        }
        // This updates the left node's next node to point to prev basically joining the reversed part
        // of the list to the node before the left node.
        *curr = prev;

        // This loop now goes over the reversed part of the list to the node after the right index.
        for _ in left..right {
            if let Some(node) = curr {
                curr = &mut node.next;
            }
        }
        
        // this sets the next node of the reversed end to the rest of the list completing the attachment of the
        // reversed part to the rest of the linked list. 
        *curr = rest;
        head
    }
}
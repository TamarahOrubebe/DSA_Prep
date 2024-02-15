/*
Remove Duplicates from Sorted List

Solution
Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

 

Example 1:


Input: head = [1,1,2]
Output: [1,2]
Example 2:


Input: head = [1,1,2,3,3]
Output: [1,2,3]
 

Constraints:

The number of nodes in the list is in the range [0, 300].
-100 <= Node.val <= 100
The list is guaranteed to be sorted in ascending order.
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;
        
        while current.as_ref().is_some() && current.as_ref().unwrap().next.is_some() {
            if current.as_ref().unwrap().val == current.as_ref().unwrap().next.as_ref().unwrap().val {
                    let next_next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                    current.as_mut().unwrap().next = next_next;
                } else {
                    current = &mut current.as_mut().unwrap().next;
            }
        }

        head
    }
}
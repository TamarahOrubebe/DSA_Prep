/*
Example 1: Given the head of a linked list with an odd number of nodes head, return the value of the node in the
middle.

For example, given a linked list that represents 1 -> 2 -> 3 -> 4 -> 5, return 3.
*/


// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn get_middle(head: Option<Box<ListNode>>) -> i32 {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.as_ref().unwrap().val
}

// MORE IDIOMATIC RUST CODE
fn get_middle(head: Option<Box<ListNode>>) -> i32 {
    let mut slow = &head;
    let mut fast = &head;

    while let Some(node) = fast {
        if node.next.is_none() {
            break;
        }
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.as_ref().unwrap().val
}

fn main() {
     // Create the nodes
    let mut head = ListNode::new(1);
    let mut second = ListNode::new(2);
    let mut third = ListNode::new(3);
    let mut fourth = ListNode::new(4);
    let fifth = ListNode::new(5);

    // Link the nodes
    fourth.next = Some(Box::new(fifth));
    third.next = Some(Box::new(fourth));
    second.next = Some(Box::new(third));
    head.next = Some(Box::new(second));
    
    // Use the linked list as input for the get_middle function
    let result = get_middle(Some(Box::new(head)));
    println!("Middle element: {}", result);
}





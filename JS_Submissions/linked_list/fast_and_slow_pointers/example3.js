/* 
Example 3: Given the head of a linked list and an integer k, return the kth node from the end.

For example, given the linked list that represents 1 -> 2 -> 3 -> 4 -> 5 and k = 2, return the node with value 4,
as it is the 2nd node from the end.

This problem is very similar to the first example. Again, we could just convert the list to an array, or we 
could iterate through once to find the length and then iterate again once we know the length, but there is a 
more elegant solution.

If we separate the two pointers by a gap of k, and then move them at the same speed, they will always be k apart.
When the fast pointer (the one further ahead) reaches the end, then the slow pointer must be at the desired node,
since it is k nodes behind.
*/

let findNode = (head, k) => {
    let slow = head;
    let fast = head;
    for (let i = 0; i < k; i++) {
        fast = fast.next;
    }

    while (fast) {
        slow = slow.next;
        fast = fast.next;
    }

    return slow;
}

// For the same reasons as in the first example, the time complexity of this algorithm is O(n) and the space
// complexity is O(1), where nn is the number of nodes in the linked list.

// Try solving these upcoming practice problems using the fast and slow pointer technique.


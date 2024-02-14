/* 
Example 1: Given the head of a linked list with an odd number of nodes head, return the value of the node in the
middle.

For example, given a linked list that represents 1 -> 2 -> 3 -> 4 -> 5, return 3.

As mentioned in the previous article, the easiest way to solve this problem would be to just convert the linked 
list into an array by iterating over it, and then just returning the number in the middle.

function fn(head):
    array = int[]
    while head:
        array.push(head.val)
        head = head.next

    return array[array.length // 2]
This is basically "cheating", and would never pass as an acceptable solution in an interview.You may have 
realized that the difficulty in this problem comes from the fact that we don't know how long the linked list is. 
One thing we could do is iterate through the linked list once with a dummy pointer to find the length, then 
iterate from the head again once we know the length to find the middle.


let getMiddle = head => {
    let length = 0;
    let dummy = head;
    while (dummy) {
        length++;
        dummy = dummy.next;
    }

    for (let i = 0 ; i < Math.floor(length / 2); i++) {
        head = head.next;
    }

    return head.val;
}


The most elegant solution comes from using the fast and slow pointer technique. If we have one pointer moving 
twice as fast as the other, then by the time it reaches the end, the slow pointer will be halfway through since 
it is moving at half the speed.

Click here for a more detailed explanation if needed
Remember that slow and fast are completely independent of each other.

In each while loop iteration we move fast forward two steps, so it will take n/2 iterations for it to reach the 
end. In each iteration, slow moves forward one step, so after n/2 iterations, slow will be at the middle, which 
is exactly what we want.

Don't forget that when using fast.next.next, you should also make sure fast.next is not null to avoid any errors.

*/

const getMiddle = head => {
    let slow = head;
    let fast = head;
    
    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;
    }
    
    return slow.val;
}

// The pointers use O(1) space, and if there are nn nodes in the linked list, the time complexity is O(n)
// for the traversals.





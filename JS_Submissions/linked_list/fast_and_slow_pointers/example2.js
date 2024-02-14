/*
Example 2: 141. Linked List Cycle

Given the head of a linked list, determine if the linked list has a cycle.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously 
following the next pointer.

If a linked list has a cycle, you can imagine some group of nodes forming a circle, and traversal never ends as 
it moves around that circle infinitely. One way to try to solve this problem would be to just iterate through 
the list for an arbitrarily large amount of iterations. If there isn't a cycle, then we will eventually reach 
the end of the list. If there is a cycle, then we will never reach an end and after a huge amount of iterations, 
we can conclude that there is probably a cycle.

The problem with this approach is that it isn't an actual general solution. What if there is no cycle, but there 
just happens to be more nodes than the iteration cutoff? If we increase the iteration cutoff, we can always 
argue that we could pass in a longer linked list. If we make the cutoff too large, it becomes impractical, 
and we are hard coding which is a terrible practice.

The better approach is to use a fast and slow pointer. Imagine a straight racetrack (like the one used in the 
100m sprint). If two runners of significantly different speeds are racing, then the slower one will never 
catch up to the faster one. The faster runner finishing the race is like the fast pointer reaching the end 
of the linked list.

But what if the runners were instead running around a circular racetrack, and needed to complete many laps? In 
that case, the faster runner will eventually pass (lap) the slower runner.

We can apply this logic - move a fast pointer twice the speed of a slow pointer. If they ever meet (except at 
the start), then we know there must be a cycle. If the fast pointer reaches the end of the linked list, then
there isn't a cycle.

Why will the pointers always meet, and the fast pointer won't just "skip" over the slow pointer in the cycle? 
After looping around the cycle for the first time, if the fast pointer is one position behind, then the pointers 
will meet on the next iteration. If the fast pointer is two positions behind, then it will be one position 
behind on the next iteration. This pattern continues - after looping around once, the fast pointer moves exactly
one step closer to the slow pointer at each iteration, so it's impossible for it to "skip" over.


*/

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycle = function(head) {
    let slow = head;
    let fast = head;
    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;
        if (slow == fast) {
            return true;
        }
    }
    
    return false;
};

/* 
This approach gives us a time complexity of O(n)O(n) and a space complexity of O(1)O(1), where n is the number 
of nodes in the linked list. Note that this problem can also be solved using hashing, although it would require 
O(n)O(n) space.

The hashing solution: if you continuously iterate using the next pointer, there are two possibilities:

If the linked list doesn't have a cycle, you will eventually reach null and finish.
If the linked list has a cycle, you will eventually visit a node twice. We can use a set to detect this.
*/

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycle = function(head) {
    let seen = new Set();
    while (head) {
        if (seen.has(head)) {
            return true;
        }
        
        seen.add(head);
        head = head.next;
    }
    
    return false;
};

// This solution is added for the sake of completeness - the first one is better as it uses less space.


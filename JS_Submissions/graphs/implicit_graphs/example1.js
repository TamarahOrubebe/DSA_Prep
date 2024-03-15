/*
Example 1: 752. Open the Lock

You have a lock with 4 circular wheels. Each wheel has the digits 0 to 9. The wheels rotate and wrap around - so 
0 can turn to 9 and 9 can turn to 0. Initially, the lock reads "0000". One move consists of turning a wheel one 
slot. You are given an array of blocked codes deadends - if the lock reads any of these codes, then it can no 
longer turn. Return the minimum number of moves to make the lock read target.

Whenever a problem is asking for the minimum number of steps/operations/moves to do something, you should 
immediately think if BFS could be used.

We can think about each number in the range [0, 9999] as a node. For a given node, the neighbors are all numbers 
that differ in only one position by a value of one (while considering the wrap-around case of 0 and 9).

Let's use a helper function neighbors that takes a node and generates all strings that differ in one position by 
a value of one.

Once we have this function, the implementation comes down to a very simple BFS that we have done many times 
already. Associate the number of steps taken so far with each node in the queue. For each (node, steps) pair, 
if node = target, then return steps. Otherwise, iterate over neighbors(node) and for each neighbor, check if it 
has been visited already (using a set seen). If it hasn't, push (neighbor, steps + 1) onto the queue. We start 
the BFS from "0000".

One last thing: we have an added restriction where we can't visit any nodes in deadends. We can initialize seen 
with all these nodes since seen already provides us with the function of not visiting nodes.


*/

/**
 * @param {string[]} deadends
 * @param {string} target
 * @return {number}
 */
var openLock = function(deadends, target) {
    let neighbors = node => {
        let ans = [];
        for (let i = 0; i < 4; i++) {
            let num = node[i];
            for (const change of [-1, 1]) {
                let x = (+num + change + 10) % 10
                ans.push(node.slice(0, i) + x + node.slice(i + 1));
            }
        }
        
        return ans;
    }
    
    if (deadends.includes("0000")) {
        return -1;
    }
    
    let queue = ["0000"];
    let seen = new Set(deadends);
    seen.add("0000");
    
    let steps = 0;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        
        for (let i = 0; i < currentLength; i++) {
            const node = queue[i];
            if (node == target) {
                return steps;
            }

            for (const neighbor of neighbors(node)) {
                if (!seen.has(neighbor)) {
                    seen.add(neighbor);
                    nextQueue.push(neighbor);
                }
            }
        }
        
        steps++;
        queue = nextQueue;
    }
    
    return -1;
};

/*
Technically, the time complexity of this algorithm is O(d)O(d), where d = deadends.length for converting deadends
into a set. This is because everything else in the problem is constant (4 slots, 10 digits). However, if the lock
had a variable number of slots, let's say n, then this changes our time complexity to O(10^n . n^2 + d)O(10 
There are 10^n different states because each slot has 10 options. At each state, we perform O(n^2) work because 
we loop over the n slots while performing string concatenation, which is O(n) for immutable strings.
*/


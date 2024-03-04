/*
Just like DFS, the code/implementations for BFS is very similar across different problems. Here is a general 
format (we're printing the values of the nodes as an example):

let printAllNodes = root => {
    let queue = [root];
    while (queue.length) {
        let nodesInCurrentLevel = queue.length;
        let nextQueue = [];

        for (let i = 0; i < nodesInCurrentLevel; i++) {
            let node = queue[i];

            // do some logic here on the current node
            console.log(node.val);

            // put the next level onto the queue
            if (node.left) {
                nextQueue.push(node.left);
            }
            if (node.right) {
                nextQueue.push(node.right);
            }
        }

        queue = nextQueue;
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
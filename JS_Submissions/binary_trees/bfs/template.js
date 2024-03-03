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
*/
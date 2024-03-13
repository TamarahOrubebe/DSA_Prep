/*
Example 2: 863. All Nodes Distance K in Binary Tree

Given the root of a binary tree, a target node target in the tree, and an integer k, return an array of the 
values of all nodes that have a distance k from the target node.

In a binary tree, we only have pointers from parents to children. We can easily find the nodes at distance k that
are in the target node's subtree, but what about all the other nodes? Let's convert the tree into a graph by 
assigning every node a parent pointer. Then, the tree becomes an undirected graph, and we can use a simple BFS 
to find the nodes at distance k.

We can perform the parent assignments using either BFS or DFS - it doesn't really matter, so we'll use DFS. Then,
we'll perform a BFS starting at target, and after we have reached k steps, we will return the nodes in the queue.

If the target was the root, then this problem is easy - we just do BFS until we reach the k^th level, and then we
know that our queue will be holding all the answer nodes.

Unfortunately, target could be any node. Even worse, we aren't just considering nodes in the subtree of target, 
but any node! Normally, it is impossible for us to traverse to any nodes outside of a starting node's subtree, 
because we have no way of moving back up to the parent.

To solve this, we can first perform a standard DFS from the root and mark each node with its parent. We can do 
this either with a hash map or by modifying the object with a parent attribute. To perform the marking, we use 
an argument parent along with the standard node argument in our recursive DFS function. We set the parent of 
node to parent, and then when we call on the children, we pass node as parent.

Once we have the parents marked, we created an undirected graph (as we can move to parents and children now). 
Now, we can just do a BFS starting from target. Using the same format as we did in the binary tree problems, we 
have each while loop iteration handle a whole level. After kk iterations, we know our queue will hold all the 
answer nodes.
*/

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {TreeNode} target
 * @param {number} k
 * @return {number[]}
 */
var distanceK = function(root, target, k) {
    let dfs = (node, parent) => {
        if (!node) {
            return;
        }
        
        node.parent = parent;
        dfs(node.left, node);
        dfs(node.right, node);
    }
    
    dfs(root, null);
    let queue = [target];
    let seen = new Set([target]);
    let distance = 0;
    
    while (queue.length && distance < k) {
        let currentLength = queue.length;
        let nextQueue = [];
        
        for (let i = 0; i < currentLength; i++) {
            let node = queue[i];
            for (const neighbor of [node.left, node.right, node.parent]) {
                if (neighbor && !seen.has(neighbor)) {
                    seen.add(neighbor);
                    nextQueue.push(neighbor);
                }
            }
        }
        
        queue = nextQueue;
        distance++;
    }
    
    return queue.map(node => node.val);
};

// Both the DFS and BFS perform constant work at each node, and only visit each node at most once. Therefore we 
// have a time and space complexity of O(n) (the space comes from the recursion call stack when we assign the 
// parents, the queue, and seen).
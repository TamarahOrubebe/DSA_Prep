/*
Example 4: 1293. Shortest Path in a Grid with Obstacles Elimination

You are given an m x n integer matrix grid where each cell is either 0 (empty) or 1 (obstacle). You can move up, 
down, left, or right from and to an empty cell in one step. Return the minimum number of steps to walk from the 
upper left corner to the lower right corner given that you can eliminate at most k obstacles. If it is not 
possible, return -1.

This problem has two main differences. First, we can't move diagonally (this is trivial to handle, we just modify
directions to not include diagonal deltas). Second, we are allowed to move to squares with 1, but only up to k.

Recall that when we were looking at binary trees, we occasionally passed arguments to our dfs function other than
node. This was how we associated crucial information with each node. For example, when we looked at Path Sum, we 
passed an argument curr that represented the sum of the path we have taken so far.

In the first and third examples in this article, we associated the current level (as an integer steps) with each 
node.

This idea of associating additional information with nodes is a very common and useful one. In this problem, we 
are allowed to remove up to k obstacles on a given path. We can use a variable remain to represent how many 
removals we have remaining on the current path. Initially, we start in the top left with remain = k. For every
(node, remain) pair, we consider the neighbors like usual. If a neighbor is 0, then we just move to it without 
modifying remain. If a neighbor is 1, we can move to it, but we use up one of our removals, so we pass remain - 1
with the neighbor. Of course, we can only do this if remain > 0.

We've been using seen to avoid visiting the same node twice. In reality, seen actually prevents us from visiting 
the same state twice. It's just that we've only looked at problems where the node entirely describes the state. 
We need to store (node, remain) in seen instead of just node.



*/

/**
 * @param {number[][]} grid
 * @param {number} k
 * @return {number}
 */
var shortestPath = function(grid, k) {
    let valid = (row, col) => {
        return 0 <= row && row < m && 0 <= col && col < n;
    }
    
    let m = grid.length;
    let n = grid[0].length;
    let queue = [[0, 0, k]];
    let seen = [];
    for (let i = 0; i < m; i++) {
        seen.push([]);
        for (let j = 0 ; j < n; j++) {
            seen[i].push(-1);
        }
    }
    
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let steps = 0;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        
        for (let i = 0; i < currentLength; i++) {
            let [row, col, remain] = queue[i];
            if (row == m - 1 && col == n - 1) {
                return steps;
            }

            // if the current square is an obstacle, we need to spend one of our removals
            if (grid[row][col] == 1) {
                if (remain == 0) {
                    continue;
                } else {
                    remain--;
                }
            }

            // if the square has already been visited, but with more removals, then the current
            // path is pointless, since more removals is better
            if (seen[row][col] >= remain) {
                continue;
            }

            seen[row][col] = remain;
            for (const [dx, dy] of directions) {
                let nextRow = row + dy, nextCol = col + dx;
                if (valid(nextRow, nextCol)) {
                    nextQueue.push([nextRow, nextCol, remain]);
                }
            }
        }
        
        queue = nextQueue;
        steps++;
    }
    
    return -1;
};

/* 
We said that the time complexity for graph algorithms is O(nodes + edges). The argument was that we never visited
a node more than once. Technically, we should be using ss instead of nodes, where ss denotes the number of states,
and we never visit a state more than once due to seen.

It's just that for all the problems we have looked at so far, the node entirely described a state. Therefore, we 
always had s = nodes.

In this problem, we have two variables representing a state - (node, remain). There are m \cdot nm⋅n values for 
node and kk values for remain. This gives us m \cdot n \cdot km⋅n⋅k states.

The work done at each state is O(1), which gives us a time complexity of O(m . n . k). The space complexity is 
the same, as seen grows linearly with the number of states.
*/
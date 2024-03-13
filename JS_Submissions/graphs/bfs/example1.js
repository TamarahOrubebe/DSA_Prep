/*
Example 1: 1091. Shortest Path in Binary Matrix

Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no 
clear path, return -1. A clear path is a path from the top-left cell (0, 0) to the bottom-right cell (n-1, n-1)
such that all visited cells are 0. You may move 8-directionally (up, down, left, right, or diagonally).


In this problem, we are asked to find a "clear path". A clear path is a path that starts in the top left, ends in
the bottom right, and contains only 0.

Therefore, we have a graph where each square with value 0 is a node, and edges exist between nodes that are 
adjacent horizontally, vertically, or diagonally.

As shown in the image above, starting a DFS at the top left and traversing until you reach the bottom right may 
produce an incorrect answer.

With BFS, every time we visit a node, we must have arrived in the fewest possible steps. When we reach the 
bottom right, its guaranteed that we did so in the fewest possible steps. If we associate the steps taken so far
with each node, then we can immediately return once we reach the bottom right.

We store an extra integer with a node in each queue entry. When we get a node from the queue, we also get its 
steps. When we put the neighbors of node onto the queue, we also push steps + 1.

The implementation should look very familiar. We use a helper function valid and a directions array to make the 
code cleaner, a good practice for all matrix graph problems. The BFS is identical to the iterative DFS 
implementations - each iteration in the while loop is handling a single node. The only difference between DFS 
and BFS is that we are using a queue instead of a stack.
*/

/**
 * @param {number[][]} grid
 * @return {number}
 */
var shortestPathBinaryMatrix = function(grid) {
    let valid = (row, col) => {
        return 0 <= row && row < n && 0 <= col && col < n && grid[row][col] == 0;
    }
    
    if (grid[0][0] == 1) {
        return -1;
    }
    
    let n = grid.length;
    let seen = [];
    for (let i = 0; i < n; i++) {
        seen.push(new Array(n).fill(false));
    }
    seen[0][0] = true;
    
    let queue = [[0, 0]]; // row, col
    let directions = [[0, 1], [1, 0], [1, 1], [-1, -1], [-1, 1], [1, -1], [0, -1], [-1, 0]];
    let steps = 0;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        steps++;
        
        for (let i = 0; i < currentLength; i++) {
            let [row, col] = queue[i];
            if (row == n - 1 && col == n - 1) {
                return steps;
            }

            for (const [dx, dy] of directions) {
                let nextRow = row + dy, nextCol = col + dx;
                if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                    seen[nextRow][nextCol] = true;
                    nextQueue.push([nextRow, nextCol]);
                }
            }
        }
        
        queue = nextQueue;
    }
    
    return -1;
};

// If the queue implementation is efficient, then removing from the left is O(1) which makes the work at each 
// node O(1).This means the time complexity is equal to the number of nodes, which is O(n ^ 2). The space 
// complexity is also O(n ^ 2) as seen can grow to that size.
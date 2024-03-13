/*
Example 3: 542. 01 Matrix

Given an m x n binary (every element is 0 or 1) matrix mat, find the distance of the nearest 0 for each cell. 
The distance between adjacent cells (horizontally or vertically) is 1.

For example, given mat = [[0,0,0],[0,1,0],[1,1,1]], return [[0,0,0],[0,1,0],[1,2,1]].

We have another matrix graph. We are already familiar with traversing these with the help of tools like valid and
directions.

An important observation to make: there is no difference between starting at a 1 and looking for a 0, or starting
from a 0 and looking for a 1.

Let's say you are focusing on a 1 at square x. We do a BFS and find the nearest 0 is at a square y.

Now, let's try starting a BFS from every 0 looking for x. After performing many BFS, you find that the one that 
produced a minimum distance was y.

This leads us to our solution. In all the BFS examples we have looked at so far in the course, we initialize the 
queue with only one node - the node we start our BFS from. This single node represented the 0^th level - the 
nodes that have a distance of 0 from the source. There is nothing stopping us from having multiple nodes in the 
0^th level.

We said that with BFS, every time we visit a node, we did so in the fewest steps possible from the source.

The "source" is actually the 0^th level - not a single node. It's just that so far, we have only looked at 
problems where the 0^th level had only one node.

In this problem, we can have the "source" be any node with a value of 0. We do this by initializing queue with 
all 0 nodes. Again, we should associate the steps taken so far (the level) with each node. By the definition of 
BFS, every time we visit a node, we will have done so in the fewest steps possible from a 0, which is exactly 
what the problem is asking for. By using seen, we will not override any shortest distances we have already found.

*/

/**
 * @param {number[][]} mat
 * @return {number[][]}
 */
var updateMatrix = function(mat) {
    let valid = (row, col) => {
        return 0 <= row && row < m && 0 <= col && col < n && mat[row][col] == 1;
    }
    
    // if you don't want to modify the input, you can create a copy at the start
    m = mat.length;
    n = mat[0].length;
    let queue = [];
    let seen = [];
    for (let i = 0; i < m; i++) {
        seen.push(new Array(n).fill(false));
    }
    
    for (let row = 0; row < m; row++) {
        for (let col = 0; col < n; col++) {
            if (mat[row][col] == 0) {
                queue.push([row, col]);
                seen[row][col] = true;
            }
        }
    }
    
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let steps = 0;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        steps++;
        
        for (let i = 0; i < currentLength; i++) {
            const [row, col] = queue[i];
            for (const [dx, dy] of directions) {
                let nextRow = row + dy, nextCol = col + dx;
                if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                    seen[nextRow][nextCol] = true;
                    nextQueue.push([nextRow, nextCol]);
                    mat[nextRow][nextCol] = steps;
                }
            }
        }
        
        queue = nextQueue;
    }
    
    return mat;
};

// his algorithm improves the time complexity to O(m \cdot n)O(m⋅n), because the BFS now only visits each square 
// once, and does a constant amount of work each time.The space complexity is also O(m . n) for the queue and 
// seen.
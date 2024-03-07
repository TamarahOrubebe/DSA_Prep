/*
Example 2: 200. Number of Islands

Given an m x n 2D binary grid which represents a map of 1 (land) and 0 (water), return the number of islands. 
An island is surrounded by water and is formed by connecting adjacent land cells horizontally or vertically.

A matrix is a very common form of graph. We treat each land cell as a node, and the edges are determined by the 
problem description.

It says that an island is formed by connecting adjacent land cells horizontally or vertically. Therefore, two 
land cells share an edge if they are adjacent. For a node at (row, col), the neighbors are at (row - 1, col),
(row, col - 1), (row + 1, col), (row, col + 1) (if in bounds).

In an island, you can start at any land cell and reach any other land cell. We just saw this exact same idea in 
the previous problem - an island is like a province.

We have identified that this is the same problem - find the number of islands = find the number of connected 
components. The only thing that has changed is the format in which the input provides us with the graph.

In the code, we have a few tools to help us implement the algorithm. First, we declare an array 
directions = [(0, 1), (1, 0), (0, -1), (-1, 0)] which holds the coordinate deltas to move in the four directions.
This makes the code cleaner when iterating over the neighbors. Next, we use a helper function valid that checks 
if a square is in bounds and an island. While these tools aren't necessary, they make the code cleaner and more 
modular.
*/

/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
    let valid = (row, col) => {
        return 0 <= row && row < m && 0 <= col && col < n && grid[row][col] == "1";
    }
    
    let dfs = (row, col) => {
        for (const [dx, dy] of directions) {
            let nextRow = row + dy, nextCol = col + dx;
            if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                seen[nextRow][nextCol] = true;
                dfs(nextRow, nextCol);
            }
        }
    };
    
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let m = grid.length;
    let n = grid[0].length;
    let seen = [];
    
    for (let i = 0; i < m; i++) {
        seen.push(new Array(n).fill(false));
    }
    
    let ans = 0;
    for (let row = 0; row < m; row++) {
        for (let col = 0; col < n; col++) {
            if (grid[row][col] == "1" && !seen[row][col]) {
                ans++;
                seen[row][col] = true;
                dfs(row, col);
            }
        }
    }
    
    return ans;
};

/*
Some code differences: we only care about squares whose value is "1" (land). We can define a helper function 
valid that first checks if a given (row, col) is in bounds, and then checks if it is land. We can also declare 
an array directions that makes iterating over the 4 neighbors cleaner (this is a very common practice).

We said before that DFS on a graph has a time complexity of O(nodes + edges). This was because we didn't know 
how many edges a given node could have. However, here the problem explicitly defines that a node can have 
(at most) 4 edges. Therefore, the work done at each node is once again O(1), and since 
we are only visiting each node once, the time complexity is equal to the number of nodes, which is O(m . n)

*/
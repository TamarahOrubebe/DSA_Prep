/*
Max Area of Island

Solution
You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

The area of an island is the number of cells with a value 1 in the island.

Return the maximum area of an island in grid. If there is no island, return 0.

 

Example 1:


Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
Output: 6
Explanation: The answer is not 11, because the island must be connected 4-directionally.
Example 2:

Input: grid = [[0,0,0,0,0,0,0,0]]
Output: 0
 

Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 50
grid[i][j] is either 0 or 1.
*/

/**
 * @param {number[][]} grid
 * @return {number}
 */
var maxAreaOfIsland = function(grid) {
    let isValid = (row, col) => {
        return 0 <= row && row < m && 0 <= col && col < n && grid[row][col] == "1";
    }
    
    
    let dfs = (row, col) => {
        if(!isValid(row, col)) {
            return 0;
        }
        let area = 1;
        
        for(const [x, y] of directions) {
            let newRow = row + y;
            let newCol = col + x;
        
            if(isValid(newRow, newCol) && !seen[newRow][newCol]) {
                 seen[newRow][newCol] = true;
                area += dfs(newRow, newCol);
            } 
            
        }
        return area;
    }
    
    let m = grid.length;
    let n = grid[0].length;
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    
    let seen = [];
    
    
    for(let i = 0; i < m; i++) {
        seen.push(new Array(n).fill(false));
    }
    
    let maxArea = 0;
    for(let row = 0; row < m; row++) {
        for(let col = 0; col < n; col++) {
            if(grid[row][col] == "1" && !seen[row][col]) {
                seen[row][col] = true;
                maxArea = Math.max(maxArea, dfs(row, col));
            }
        }
        

    }
    
    return maxArea;
};

// time and space complexities of O(m . n) 
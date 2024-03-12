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



fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        
           fn dfs(row: i32, col: i32, grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>) -> i32 {
                fn is_valid(row: i32, col: i32, grid: &Vec<Vec<i32>>) -> bool {
                    let m = grid.len() as i32;
                    let n = grid[0].len() as i32;
                    0 <= row && row < m && 0 <= col && col < n && grid[row as usize][col as usize] == 1
                }

                if !is_valid(row, col, grid) {
                    return 0;
                }
               
                let mut area = 1;
                let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
                for (dx, dy) in &directions {
                    let next_row = row + dy;
                    let next_col = col + dx;

                    if is_valid(next_row, next_col, grid) && !seen[next_row as usize][next_col as usize] {
                        seen[next_row as usize][next_col as usize] = true;
                        area += dfs(next_row, next_col, grid, seen)
                    }
                }
            area    
        }
        
        let m = grid.len();
        let n = grid[0].len();
        let mut seen = vec![vec![false; n]; m];
        let mut max_area = 0;

        for row in 0..m {
            for col in 0..n {
                if grid[row][col] == 1 {
                    seen[row][col] = true;
                    max_area = max_area.max(dfs(row as i32, col as i32, &grid, &mut seen));
                }
            }
        }
        max_area
}

// time and space complexities O(m . n);
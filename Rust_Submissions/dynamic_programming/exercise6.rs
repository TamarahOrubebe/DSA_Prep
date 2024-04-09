/*
Unique Paths II

Solution
You are given an m x n integer array grid. There is a robot initially located at the top-left corner 
(i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can
only move either down or right at any point in time.

An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any
square that is an obstacle.

Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The testcases are generated so that the answer will be less than or equal to 2 * 109.

 

Example 1:


Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
Output: 2
Explanation: There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right
Example 2:


Input: obstacleGrid = [[0,1],[0,0]]
Output: 1
 

Constraints:

m == obstacleGrid.length
n == obstacleGrid[i].length
1 <= m, n <= 100
obstacleGrid[i][j] is 0 or 1.
*/

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        
        fn dp(obstacle_grid: &Vec<Vec<i32>>, row: usize, col: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if row + col == 0 && obstacle_grid[row][col] != 1 {
                return 1;
            }
            
            if let Some(val) = memo[row][col] {
                return val;
            }
            
            let mut ways = 0;
            if row > 0 && obstacle_grid[row][col] != 1 {
                ways += dp(obstacle_grid, row -1, col, memo);
            }
            
            if col > 0 && obstacle_grid[row][col] != 1 {
                ways += dp(obstacle_grid, row, col - 1, memo);
            }
            
            memo[row][col] = Some(ways);
            ways
        }
        
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut memo = vec![vec![None; n]; m];
        dp(&obstacle_grid, m - 1, n - 1, &mut memo)
    }
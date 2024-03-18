/*
Nearest Exit from Entrance in Maze

Solution
You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.

In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance does not count as an exit.

Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.

 

Example 1:


Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
Output: 1
Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
Initially, you are at the entrance cell [1,2].
- You can reach [1,0] by moving 2 steps left.
- You can reach [0,2] by moving 1 step up.
It is impossible to reach [2,3] from the entrance.
Thus, the nearest exit is [0,2], which is 1 step away.
Example 2:


Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
Output: 2
Explanation: There is 1 exit in this maze at [1,2].
[1,0] does not count as an exit since it is the entrance cell.
Initially, you are at the entrance cell [1,0].
- You can reach [1,2] by moving 2 steps right.
Thus, the nearest exit is [1,2], which is 2 steps away.
Example 3:


Input: maze = [[".","+"]], entrance = [0,0]
Output: -1
Explanation: There are no exits in this maze.
 

Constraints:

maze.length == m
maze[i].length == n
1 <= m, n <= 100
maze[i][j] is either '.' or '+'.
entrance.length == 2
0 <= entrancerow < m
0 <= entrancecol < n
entrance will always be an empty cell.
*/

use std::collections::VecDeque;

fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        
        fn is_valid(row: i32, col: i32, maze: &Vec<Vec<char>>) -> bool {
            let m = maze.len() as i32;
            let n = maze[0].len() as i32;
            0 <= row && row < m && 0 <= col && col < n && maze[row as usize][col as usize] == '.'
        }
        let directions: Vec<Vec<i32>> = vec![vec![0,1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        let mut seen: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();
        queue.push_back(entrance.clone());
        let x = &entrance[0];
        let y = &entrance[1];
        seen[*x as usize][*y as usize] = true;
        let mut steps = 0;
        while !queue.is_empty() {
            let curr_length = queue.len();
            
            for _ in 0..curr_length {
                if let Some(node) = queue.pop_front() {
                    let row = node[0 as usize];
                    let col = node[1 as usize];

                    for coord in directions.iter() {
                        let dx = coord[0 as usize];
                        let dy = coord[1 as usize];
                        let new_row = row + dy;
                        let new_col = col + dx;

                        if is_valid(new_row, new_col, &maze) && !seen[new_row as usize][new_col as usize] {

                            if new_row == 0 || new_row == (m as i32) - 1 || new_col == 0 || new_col == (n as i32) - 1 {
                                 return steps + 1;
                            }
                            
                            seen[new_row as usize][new_col as usize] = true;
                            queue.push_back(vec![new_row, new_col]);
                        }
                    }
                }
            }
            steps += 1;
        }
        -1
}

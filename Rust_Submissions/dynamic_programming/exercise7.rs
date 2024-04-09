/*
Minimum Falling Path Sum

Solution
Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.

A falling path starts at any element in the first row and chooses the element in the next row that is either 
directly below or diagonally left/right. Specifically, the next element from position (row, col) will be 
(row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).

 

Example 1:


Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
Output: 13
Explanation: There are two falling paths with a minimum sum as shown.
Example 2:


Input: matrix = [[-19,57],[-40,-5]]
Output: -59
Explanation: The falling path with a minimum sum is shown.
 

Constraints:

n == matrix.length == matrix[i].length
1 <= n <= 100
-100 <= matrix[i][j] <= 100
*/

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        fn dp(matrix: &Vec<Vec<i32>>, row: usize, col: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            let n = matrix.len();
            if row == matrix.len() - 1 {
                return matrix[row][col];
            }
            
            if let Some(val) = memo[row][col] {
                return val;
            }
            
            let left = if col > 0 {
                dp(matrix, row + 1, col -1, memo)
            } else {
                i32::MAX
            };
            
            let mid = dp(matrix, row + 1, col, memo);
            
            let right = if col < n - 1 {
                dp(matrix, row + 1, col +1, memo)
            } else {
                i32::MAX
            };
            
            let ans = i32::min(i32::min(left, mid), right) + matrix[row][col];
            memo[row][col] = Some(ans);
            ans
        }
        
        let n = matrix.len();
        let mut memo = vec![vec![None; n]; n];
        let mut min_sum = i32::MAX;
        for i in 0..n {
            min_sum = min_sum.min(dp(&matrix, 0, i, &mut memo));
        }
        min_sum
    }
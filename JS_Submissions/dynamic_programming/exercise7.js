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


/**
 * @param {number[][]} matrix
 * @return {number}
 */
var minFallingPathSum = function(matrix) {
       const dp = (row, col) => {
           if(row === n - 1) {
               return matrix[row][col];
           }
           
           if(memo[row][col] != Infinity) {
               return memo[row][col];
           }
           
           
           let left = col > 0? dp(row + 1, col - 1) : Infinity;
           let mid = dp(row + 1, col);
           let right = col < n - 1 ? dp(row + 1, col + 1) : Infinity;
           
           memo[row][col] = Math.min(left, mid, right) + matrix[row][col];
        
           return memo[row][col];
       }
       
       let n = matrix.length;
       const memo = [];
        for (let i = 0; i < n; i++) {
            memo.push(new Array(n).fill(Infinity));
        }
    
       let ans = Infinity;
       for(let i = 0; i < n; i++) {
             ans = Math.min(ans, dp(0, i));  
       }
    
       return ans;
};
 

// BOTTOM UP APPROACH
// var minFallingPathSum = function(matrix) {
//         const n = matrix.length;
//         const dp = new Array(n).fill(0).map(() => new Array(n).fill(0));
        
//         for (let col = 0; col < n; col++) {
//             dp[n - 1][col] = matrix[n - 1][col]
//         }
      
        
//          for (let row = n - 2; row >= 0; row--) {
//             for (let col = 0; col < n; col++) {
//                 let left = (col > 0) ? dp[row + 1][col - 1] : Infinity;
//                 let middle = dp[row + 1][col];
//                 let right = (col < n - 1) ? dp[row + 1][col + 1] : Infinity;

//                 dp[row][col] = matrix[row][col] + Math.min(left, middle, right);
//             }
//     }
    
//     let minSum = Infinity;
//     for (let col = 0; col < n; col++) {
//         minSum = Math.min(minSum, dp[0][col]);
//     }
    
//     return minSum;
       
// };

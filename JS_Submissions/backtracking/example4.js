/*
Example 2: 52. N-Queens II

The N-Queens puzzle is the famous problem of placing n queens on an n x n chessboard such that no two queens 
attack each other. Given an integer n, return the number of distinct solutions to the N-Queens puzzle.

For those who don't play chess: a queen can attack along the row, column, and diagonals it occupies.

So far, the problems we have looked at in this chapter had very well-defined inputs that we operated on, like an 
array of integers.

In this problem, we are dealing with something a little more difficult to represent in code - a chessboard.

One way to solve this problem would be to pass an n * n matrix as an argument to the backtrack function 
(literally store the chessboard). However, this is not necessary and is very inefficient.

We should try to use as little as we need to represent a chessboard. In this problem, we are only concerned 
about queens attacking each other. When we try to place a queen, we should place it on a square that isn't 
attacked in our current state. Because a queen attacks the row, column, and diagonals it occupies, we can simply
use an argument for each one.

As mentioned above, we use a neat trick with the diagonals so that we can use a set to efficiently find if a 
diagonal is already attacked. We also only need to use an integer for the current row, and then we just place 
one queen per row.

Once we know how to represent a board, we just use the normal backtracking template. For each row, try placing a
queen on every column if the square is not attacked. "Place" a queen by updating the state variables to 
represent which column/diagonals the newly placed queen attacks. When backtracking, "remove" the queen from the
board by undoing the updates you made when placing the queen.

We know we have a valid solution when row = n (we have placed n queens).
*/

/**
 * @param {number} n
 * @return {number}
 */
var totalNQueens = function(n) {
    let backtrack = (row, diagonals, antiDiagonals, cols) => {
        // Base case - N queens have been placed
        if (row == n) {
            return 1;
        }
        
        let solutions = 0;
        for (let col = 0 ; col < n; col++) {
            let currDiagonal = row - col; // add n to avoid going negative
            let currAntiDiagonal = row + col;
            
            // If the queen is not placeable
            if (cols.has(col) || 
                diagonals.has(currDiagonal) || 
                antiDiagonals.has(currAntiDiagonal)) {
                continue;
            }
            
            // "Add" the queen to the board
            cols.add(col);
            diagonals.add(currDiagonal);
            antiDiagonals.add(currAntiDiagonal);
            
            // Move on to the next row with the updated board state
            solutions += backtrack(row + 1, diagonals, antiDiagonals, cols);
            
            // "Remove" the queen from the board since we have already
            // explored all valid paths using the above function call
            cols.delete(col);
            diagonals.delete(currDiagonal);
            antiDiagonals.delete(currAntiDiagonal);
        }
        
        return solutions;
    }
    
    return backtrack(0, new Set(), new Set(), new Set());
};


/* 
The true time complexity of this algorithm isn't actually known, but it's approximately O(n!). The argument
is that the first call will consider n options. The next call will consider n - 2 options, because one square 
will occupy the same column, and at least one square will occupy a diagonal/anti diagonal. The pattern continues,
as the next call after that will consider n - 4 squares and so on. The space complexity is O(n) due to the 
sets and the recursion call stack.
*/
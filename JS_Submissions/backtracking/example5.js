/*
Example 3: 79. Word Search

Given an m x n grid of characters board and a string word, return true if word exists in the grid. The word can 
be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically 
neighboring. The same letter cell may not be used more than once.

As we have a graph, we can perform an algorithm similar to DFS to traverse the matrix. We choose a square that 
has the value word[0] and start there. Then we look for word[1] in the neighbors. If we find it, we move to it 
and look for word[2] and so on until we complete the word or reach a point where we can't continue building the 
word.

When we realize we can't continue, we go back to the previous node (like we do in a normal DFS), but we should 
also remove the square we are leaving from the set we are using to avoid revisiting squares.

The set prevents us from using a letter more than once in a given path, but once we start abandoning a path, we 
can remove it so that it may be used in a future path.
*/

/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {
    let valid = (row, col) => {
        return 0 <= row && row < m && 0 <= col && col < n;
    }
    
    let backtrack = (row, col, i, seen) => {
        if (i == word.length) {
            return true;
        }
        
        for (const [dx, dy] of directions) {
            let nextRow = row + dy, nextCol = col + dx;
            if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                if (board[nextRow][nextCol] == word[i]) {
                    seen[nextRow][nextCol] = true;
                    if (backtrack(nextRow, nextCol, i + 1, seen)) {
                        return true;
                    }
                    seen[nextRow][nextCol] = false;
                }
            }
        }
        
        return false;
    }
    
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let m = board.length;
    let n = board[0].length;
    
    for (let row = 0; row < m; row++) {
        for (let col = 0; col < n; col++) {
            let seen = [];
            for (let i = 0; i < m; i++) {
                seen.push(new Array(n).fill(false));
            }
            
            seen[row][col] = true;
            if (board[row][col] == word[0] && backtrack(row, col, 1, seen)) {
                return true;
            }
        }
    }
    
    return false;
};

/*
The time complexity of this algorithm is O(n . m . 3^L), where L = word.length. Thinking of the solution space 
tree, each node can have 3 children (not 4, because we don't consider the square where we came from, thus each 
node has at most 3 children, except the first node). The maximum depth is L, so this gives us 3^L nodes. We also
have m . n squares that we can start from. This time complexity would occur in the worst-case scenario if we 
had a grid with only one letter, and the word was also only that one letter, plus a different letter at the end.
For example, board = [["A", "A", "A"],["A", "A", "A"],["A", "A", "A"]], word = "AAAAAAAAZ". The space complexity
is O(L) due to the recursion call stack and seen if it is a set. If using a boolean array, the space 
complexity is instead O(n⋅m).
*/
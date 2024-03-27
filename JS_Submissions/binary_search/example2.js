/*
Example 2: 74. Search a 2D Matrix

Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. Integers in 
each row are sorted from left to right. The first integer of each row is greater than the last integer of the 
previous row.

Because each row is sorted and fully less than the next row, we can treat the matrix as one array. This 
hypothetical single array has a length of m * n. If we consider the indices between [0, m * n - 1], then how do 
we find the row and column that each index converts to?

Each row has n elements. That means that row 0 is indices [0, n - 1]. Row 1 is indices [n, 2 * n - 1], and so on.
 This is equivalent to the floor division of n, aka row = i // n - the row increments every n indices.

The column can range between [0, n - 1]. Every n indices, the column resets to 0. This is perfect for the modulo
operator. col = i % n.

Then, we can just binary search on this hypothetical single array. The following image demonstrates the indices 
of this hypothetical single array on the 2d matrix.
*/

/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function(matrix, target) {
    let m = matrix.length, n = matrix[0].length;
    let left = 0, right = m * n - 1;
    
    while (left <= right) {
        let mid = Math.floor((left + right) / 2);
        let row = Math.floor(mid / n);
        let col = mid % n;
        let num = matrix[row][col];
        
        if (num == target) {
            return true;
        }
        
        if (num < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    return false;
};


/*
Because there are O(m \cdot n)O(m⋅n) elements, the initial search space has a size of O(m⋅n), which 
means this algorithm has a time complexity of O(log(m⋅n)). We don't use any extra space except for a few integer
variables.


*/
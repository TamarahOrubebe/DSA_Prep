/*
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

 

Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
Example 2:

Input: n = 1
Output: ["()"]
 

Constraints:

1 <= n <= 8
*/

/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function(n) {
    
  
    const backtrack = (path, leftCount, rightCount) => {
        if (path.length == n * 2) {
            ans.push(path.join(""));
            return;
        }
        
        if(leftCount < n) {
            path.push("(");
            backtrack(path, leftCount + 1, rightCount);
            path.pop();
            
        }
        
        if (leftCount > rightCount) {
            path.push(")");
            backtrack(path, leftCount, rightCount + 1);
            path.pop();
        }
        
    }
    
    let ans = [];
    backtrack([], 0, 0);
    return ans;
};
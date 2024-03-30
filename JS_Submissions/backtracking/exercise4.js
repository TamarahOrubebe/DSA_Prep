/*
Numbers With Same Consecutive Differences

Solution
Given two integers n and k, return an array of all the integers of length n where the difference between every 
two consecutive digits is k. You may return the answer in any order.

Note that the integers should not have leading zeros. Integers as 02 and 043 are not allowed.

 

Example 1:

Input: n = 3, k = 7
Output: [181,292,707,818,929]
Explanation: Note that 070 is not a valid number, because it has leading zeroes.
Example 2:

Input: n = 2, k = 1
Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
 

Constraints:

2 <= n <= 9
0 <= k <= 9
*/

/**
 * @param {number} n
 * @param {number} k
 * @return {number[]}
 */
var numsSameConsecDiff = function(n, k) {
     
    const backtrack = (n, num) => {
       if (n === 0) {
           ans.push(num);
           return;
       
       }
        
        const tailDigit = num % 10;
        
        const nextDigits = new Set([tailDigit + k, tailDigit - k]);
        
        for(const nextDigit of nextDigits) {
            if (0 <= nextDigit && nextDigit < 10) {
                let newNum = num * 10 + nextDigit;
                backtrack(n - 1, newNum);
            }
        }
        
    }
    const ans = [];
    for(let i = 1; i < 10; i++) {
        backtrack(n - 1, i)
    }
    return ans;
};
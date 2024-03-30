/*
Letter Combinations of a Phone Number

Solution
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number 
could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to 
any letters.


 

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
Example 2:

Input: digits = ""
Output: []
Example 3:

Input: digits = "2"
Output: ["a","b","c"]
 

Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].
*/

/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function(digits) {
    
    if (digits.length == 0) {
        return [];
    }
    let hash = new Map();
    hash.set("2", "abc");
    hash.set("3", "def");
    hash.set("4", "ghi");
    hash.set("5", "jkl");
    hash.set("6", "mno");
    hash.set("7", "pqrs");
    hash.set("8", "tuv");
    hash.set("9", "wxyz");
    
   const backtrack = (idx, path) => {
        if (path.length == digits.length) {
            ans.push(path.join(""));
            return;
        }
        
       
           const letters = hash.get(digits[idx]);
            for(const char of letters) {
                  path.push(char);
                  backtrack(idx + 1, path);
                  path.pop();   
            }
             
                     
         
    }
   
    const ans = [];
    backtrack(0, []);
    return ans;

};
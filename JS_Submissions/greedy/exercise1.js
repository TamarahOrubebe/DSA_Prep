/*
 Maximum 69 Number

Solution
You are given a positive integer num consisting only of digits 6 and 9.

Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
*/

/**
 * @param {number} num
 * @return {number}
 */

var maximum69Number  = function(num) {
    let numString = num.toString();
    
    let ans = [numString];
    for (let i = 0; i < numString.length; i++) {
        if (numString[i] == "9") {
            ans.push(numString.slice(0, i) + "6" + numString.slice(i + 1))
        } else {
            ans.push(numString.slice(0, i) + "9" + numString.slice(i + 1));
        }
    }
    
    
    ans.sort((a, b) => b - a);
    return ans[0];
};
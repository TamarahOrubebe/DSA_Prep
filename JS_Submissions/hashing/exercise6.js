/*
Maximum Number of Balloons

Solution
Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

You can use each character in text at most once. Return the maximum number of instances that can be formed.

 

Example 1:



Input: text = "nlaebolko"
Output: 1
Example 2:



Input: text = "loonbalxballpoon"
Output: 2
Example 3:

Input: text = "leetcode"
Output: 0
 

Constraints:

1 <= text.length <= 104
text consists of lower case English letters only.
*/

/**
 * @param {string} text
 * @return {number}
 */
var maxNumberOfBalloons = function(text) {
    let bCount = 0;
    let aCount = 0;
    let lCount = 0; 
    let oCount = 0; 
    let nCount = 0;
    
    for (const char of text) {
        if (char == "b") bCount++;
        else if (char == "a") aCount++;
        else if(char == "l") lCount++;
        else if(char == "o") oCount++;
        else if(char == "n") nCount++;
    }
    
    lCount = Math.floor(lCount / 2);
    oCount = Math.floor(oCount / 2);
    
    return Math.min(bCount, aCount, lCount, oCount, nCount);
   
};

// time cpmplexity O(n), space complexity O(1).
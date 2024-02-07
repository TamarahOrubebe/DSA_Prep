/* 
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a sequence of characters that can be obtained by deleting some (or none) of the
characters from the original string, while maintaining the relative order of the remaining characters. 
For example, "ace" is a subsequence of "abcde" while "aec" is not.

*/

/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
let isSubsequence = function(s, t) {
    let i = 0, j = 0;
    while (i < s.length && j < t.length) {
        if (s[i] == t[j]) {
            i++;
        }
        
        j++;
    }
    
    return i == s.length;
};

// this solution uses O(1)O(1) space. The time complexity is linear with the lengths of s and t.
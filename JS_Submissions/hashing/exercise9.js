/*
Longest Substring Without Repeating Characters

Solution
Given a string s, find the length of the longest substring without repeating characters.

 

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 
*/

/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {    
    let hash = new Map();
    let right = left = ans = 0;
    while (right < s.length) {
        hash.set(s[right], (hash.get(s[right]) || 0) + 1);
        
        while(hash.get(s[right]) > 1) {
            hash.set(s[left], hash.get(s[left]) - 1);
            left++
        }
        
        ans = Math.max(ans, right - left + 1);
        right++
    }
    
    return ans;
    
}

// time complexity O(n) because of an amortised inner while loop. space complexity O(k)



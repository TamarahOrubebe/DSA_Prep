/*
Ransom Note

Solution
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.

 

Example 1:

Input: ransomNote = "a", magazine = "b"
Output: false
Example 2:

Input: ransomNote = "aa", magazine = "ab"
Output: false
Example 3:

Input: ransomNote = "aa", magazine = "aab"
Output: true
 

Constraints:

1 <= ransomNote.length, magazine.length <= 105
ransomNote and magazine consist of lowercase English letters.
*/
/**
 * @param {string} ransomNote
 * @param {string} magazine
 * @return {boolean}
 */
var canConstruct = function(ransomNote, magazine) {
    
    const hash = new Map();
    
    for (char of magazine) {
        hash.set(char, (hash.get(char) || 0) + 1)
    }
    
    for(char of ransomNote) {
        
        if(!hash.has(char)) {
            return false;
        } 
        
        hash.set(char, hash.get(char) - 1);
        if(hash.get(char) == 0) {
            hash.delete(char);
        }
        
        
    }
    
    return true;
};

// time complexity O(m + n), space complexity O(n) 
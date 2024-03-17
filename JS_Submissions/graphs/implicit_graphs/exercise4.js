
/*Word Ladder

A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

Every adjacent pair of words differs by a single letter.
Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
sk == endWord
Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

 

Example 1:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5
Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
Example 2:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
Output: 0
Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
 

Constraints:

1 <= beginWord.length <= 10
endWord.length == beginWord.length
1 <= wordList.length <= 5000
wordList[i].length == beginWord.length
beginWord, endWord, and wordList[i] consist of lowercase English letters.
beginWord != endWord
All the words in wordList are unique.

*/

/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */
var ladderLength = function(beginWord, endWord, wordList) {
    let len = beginWord.length;
    let isValid = new Set(wordList);
    
    if(!isValid.has(endWord)) {
        return 0;
    } 
    
    const charSet = new Set(wordList.join(""));
    let neighbors = node => {
        let ans = [];
        
        for (let i = 0; i < len; i++) {
            const chars = charSet.keys();
            for (const char of chars) {
                const newWord = node.slice(0, i) + char + node.slice(i + 1);
                if(isValid.has(newWord) && newWord != node) {
                     ans.push(newWord);
                }
                   
            }
        
        }
           
        return ans;
    }
    
    
    let queue = [beginWord];
   
    
    let seen = new Set([beginWord]);
    
    
    let steps = 1;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        
        for (let i = 0; i < currentLength; i++) {
            const node = queue[i];
            if (node == endWord) {
                return steps;
            }
            
            for (const neighbor of neighbors(node)) {
                if(!isValid.has(neighbor)) continue;
                if (!seen.has(neighbor)) {
                    seen.add(neighbor);
                    nextQueue.push(neighbor);
                }
            }
        }
        
        steps++;
        queue = nextQueue;
    }
    
    return 0;
};
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

use std::collections::HashSet;
use std::collections::VecDeque;

fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut is_valid: HashSet<&String> = HashSet::new();
        for word in &word_list {
            is_valid.insert(word);
        }
        
        if !is_valid.contains(&end_word) {
            return 0;
        }
        
       
        let alphabets: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut seen = HashSet::new(); 
        seen.insert(begin_word.clone());
        queue.push_back(begin_word);
        let mut steps = 1;
        
        while !queue.is_empty() {
            let curr_len = queue.len();
            
            for i in 0..curr_len {
                if let Some(word) = queue.pop_front() {
                    if word == end_word {
                        return steps;
                    }
                   for i in 0..word.len() {
                       for ch in &alphabets {
                            let neighbor = format!("{}{}{}", &word[..i], ch, &word[i + 1..]);
                            if !is_valid.contains(&neighbor) {
                                continue;
                           }
                            if is_valid.contains(&neighbor) && !seen.contains(&neighbor) {
                                seen.insert(neighbor.clone());
                                queue.push_back(neighbor);
                            }
                        } 
                    } 
                  
                }
                
            }
            
            steps += 1;
        }
        0
}



 
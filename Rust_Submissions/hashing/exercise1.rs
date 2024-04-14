/*
Check if the Sentence Is Pangram

Solution
A pangram is a sentence where every letter of the English alphabet appears at least once.

Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.

 

Example 1:

Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
Output: true
Explanation: sentence contains at least one of every letter of the English alphabet.
Example 2:

Input: sentence = "leetcode"
Output: false
 

Constraints:

1 <= sentence.length <= 1000
sentence consists of lowercase English letters.

*/
use std::collections::HashSet;
fn check_if_pangram(sentence: String) -> bool {
    
    let mut set: HashSet<char> = sentence.chars().collect();

    set.len() == 26
}

// time complexity would be o(n) as well as the space complexity because we have to store all the unique
// alphabets in the set

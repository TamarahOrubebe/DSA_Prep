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

fn can_construct(ransom_note: String, magazine: String) -> bool {
        
        let mut hash = HashMap::new();
        
        for char in magazine.chars() {
            *hash.entry(char).or_insert(0) += 1;
        }
        
        for char in ransom_note.chars() {
            if !hash.contains_key(&char) {
                return false;
            }
            
            *hash.entry(char).or_insert(0) -= 1;
            if hash.get(&char) == Some(&0) {
                hash.remove(&char);
            }
        }
    true
}

// time complexity of O(m + n), space complexity of O(n).
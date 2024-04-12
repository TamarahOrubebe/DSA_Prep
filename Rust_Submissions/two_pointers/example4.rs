/*
Example 4: 392. Is Subsequence.

Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a sequence of characters that can be obtained by deleting some (or none) of the 
characters from the original string, while maintaining the relative order of the remaining characters. For 
example, "ace" is a subsequence of "abcde" while "aec" is not.
*/

pub fn is_subsequence(s: &str, t: &str) -> bool {
    let (mut i, mut j) = (0, 0);
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    while i < s_chars.len() && j < t_chars.len() {
        if s_chars[i] == t_chars[j] {
            i += 1;
        }
        j += 1;
    }

    i == s_chars.len()
}

//  this solution uses O(1) space. The time complexity is linear with the lengths of s and t.


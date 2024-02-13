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
use std::collections::HashMap;
fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = HashMap::new();
        let mut left = 0;
        let mut ans = 0;
        let mut vec: Vec<char> = vec![];
        
        for (i, char) in s.chars().enumerate() {   
            *hash.entry(char).or_insert(0) += 1;
            vec.push(char);
            while hash.get(&char).unwrap() > &1 {
                *hash.entry(vec[left]).or_insert(0) -= 1;
                left += 1;
            }
            let i = i as i32;
            let left = left as i32;
            ans = ans.max(i - left + 1);
               
        
        }
    ans
        
}
// time complexity O(n) because of an amortised inner while loop. space complexity O(k)
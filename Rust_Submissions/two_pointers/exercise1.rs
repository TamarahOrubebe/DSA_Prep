/*
Reverse String

Solution
Write a function that reverses a string. The input string is given as an array of characters s.

You must do this by modifying the input array in-place with O(1) extra memory.

 

Example 1:

Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]
Example 2:

Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]
 

Constraints:

1 <= s.length <= 105
s[i] is a printable ascii character.

*/
fn reverse_string(s: &mut Vec<char>) -> &mut Vec<char> {
    let mut left = 0;
    let mut right = s.len() - 1;
        
    while left < right {
        let temp = s[left];
        let second = s[right];
        [s[left], s[right]] = [s[right], s[left]];
        left += 1;
        right -= 1;
    }
    s 
}

// this algorithm has a time complexity of O(n) and a space complexity of O(1).
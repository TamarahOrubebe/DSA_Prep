/*
Example 1: Given a string s, return true if it is a palindrome, false otherwise.

A string is a palindrome if it reads the same forward as backward. That means, after reversing it, it is still 
the same string. For example: "abcdcba", or "racecar".
*/

pub fn check_if_palindrome(s: String) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s.chars().nth(left) != s.chars().nth(right) {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

/*
Notice that if the input was an array of characters instead of a string, the algorithm wouldn't change. The two 
pointers technique works as long as the index variables are moving along some abstract iterable.

This algorithm is very efficient as not only does it run in O(n), but it also uses only O(1) space. No 
matter how big the input is, we always only use two integer variables. The time complexity is O(n) because 
the while loop iterations cost O(1) each, and there can never be more than O(n) iterations of the while 
loop - the pointers start at a distance of nn from each other and move closer by one step each iteration.
*/
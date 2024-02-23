/*
Example 3: 844. Backspace String Compare

Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means 
a backspace character.

For example, given s = "ab#c" and t = "ad#c", return true. Because of the backspace, the strings are both equal 
to "ac".

Once again, we can recognize that the backspace follows the LIFO pattern, where the first character to be 
deleted is the one that was most recently typed. We can just simulate the typing of the strings using a stack, 
and then compare them at the end.

When typing characters, push them onto a stack. Whatever character is at the top of the stack is the most 
recently typed character, so when we backspace, we can just pop. Make sure to be careful of the edge case where 
we backspace on an empty string.
*/

/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var backspaceCompare = function(s, t) {
    let build = s => {
        let stack = [];
        for (const c of s) {
            if (c != "#") {
                stack.push(c);
            } else if (stack.length) {
                stack.pop();
            }
        }
        
        return stack.join("");
    }
    
    return build(s) == build(t);
};

// This algorithm has a time and space complexity of O(n), where nn is the length of the input. 
 // This is because the stack operations in all implementations above are O(1), and the stacks themselves can 
 // grow to O(n) size.
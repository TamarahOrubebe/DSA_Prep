/* 
String problems
Report Issue
String questions involving stacks are popular. Normally, string questions that can utilize a stack will involve 
iterating over the string and putting characters into the stack, and comparing the top of the stack with the 
current character at each iteration. Stacks are useful for string matching because it saves a "history" of the 
previous characters. Let's look at some examples.

Example 1: 20. Valid Parentheses

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string 
is valid. The string is valid if all open brackets are closed by the same type of closing bracket in the correct 
order, and each closing bracket closes exactly one open bracket.

For example, s = "({})" and s = "(){}[]" are valid, but s = "(]" and s = "({)}" are not valid.
*/

/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    let stack = [];
    const matching = {
        "(": ")",
        "[": "]",
        "{": "}"
    }
    
    for (const c of s) {
        if (c in matching) { // if c is an opening bracket
            stack.push(c);
        } else {
            if (!stack.length) {
                return false;
            }
            
            let previousOpening = stack.pop();
            if (matching[previousOpening] != c) {
                return false;
            }
        }
    }
    
    return !stack.length;
};

/* 
Because the stack's push and pop operations are O(1), this gives us a time complexity of O(n), where n 
is the size of the input array. This is because each element can only be pushed or popped once. The space 
complexity is also O(n) because the stack's size can grow linearly with the input size.

The key to recognizing the stack solution for this problem is seeing that the problem follows a LIFO nature, 
where the last (most recent) opening bracket is the first to be closed.
*/
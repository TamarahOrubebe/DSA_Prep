/* 
Example 2: 1047. Remove All Adjacent Duplicates In String

You are given a string s. Continuously remove duplicates (two of the same character beside each other) until you
can't anymore. Return the final string after this.

For example, given s = "abbaca", you can first remove the "bb" to get "aaca". Next, you can remove the "aa" to 
get "ca". This is the final answer.

The tricky part of this problem is that not all removals are necessarily available at the start. As you can see 
in the example, deleting the "aa" is only possible after deleting the "bb". We don't need to delete the first 
character until we have already iterated quite a bit past it. What if the input was s = "abccba"? We have the 
same problem with the b now as well, and the a is two layers back. The deletion order is c -> b -> a. This 
follows the LIFO pattern, where the last (most recent) character is the first one to be deleted 
(the first half of characters being deleted is "abc", and we need to delete the c, then b, then a).
*/

/**
 * @param {string} s
 * @return {string}
 */
var removeDuplicates = function(s) {
    let stack = [];
    for (const c of s) {
        if (stack.length && stack[stack.length - 1] == c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    
    return stack.join("");
};

 // This algorithm has a time and space complexity of O(n), where nn is the length of the input. 
 // This is because the stack operations in all implementations above are O(1), and the stacks themselves can 
 // grow to O(n) size.
/* 
Example 1: 739. Daily Temperatures

Given an array of integers temperatures that represents the daily temperatures, return an array answer such that 
answer[i] is the number of days you have to wait after the i^{th}i 
th
  day to get a warmer temperature. If there is no future day that is warmer, have answer[i] = 0 instead.

The brute force approach would be to iterate over the input, and for each temperature, iterate through the rest 
of the array until we find a warmer temperature. Let's say we had temperatures = [34, 33, 32, 31, 30, 50]. The 
first 5 days all share the same "answer" day, the 6th day. Can we leverage this fact to improve from an 
O(n^2) time complexity?

The second element 33 is not warmer than the first element 34. The third element 32 is not warmer than the 
second element 33. This property is transitive, and it implies that the third element is not warmer than the 
first element (32 \ngtr 33 \ngtr 3432≯33≯34). This means there's no point in worrying about the first element 
until we have found a warmer temperature than the second element because any temperature that isn't warmer than 
the second element is also not warmer than the first element.

This logic of handling elements in backward order should remind you of a stack. We can push the temperatures 
onto a stack, and pop them off once we find a warmer temperature. Let's look at another example, 
temperatures = [40, 35, 32, 37, 50]. Once we get to the 4th element, we have stack = [40, 35, 32]. 
Now, we see that 37 > 32 and 37 > 35, so we can pop both of them off the stack. 
This leaves us with stack = [40, 37] after pushing the 37. At the 50, we can pop both elements off the 
stack because 50 is greater than both of them.



Because the stack is monotonically decreasing, we are guaranteed to pop elements only when we find the first 
warmer temperature.

The problem wants the distance between elements, so we can store the indices instead of the actual temperatures.

Click here for a more detailed explanation if needed
Recall the problems that we looked at in the "String problems" article. All the problems shared a common theme: 
elements needed to be "operated" on in some way (verified, deleted, etc.), but not necessarily immediately.

For example, in the problem Valid Parentheses, we needed to verify opening brackets, but not immediately as we 
saw them. If we had "([{, then the ( needs to wait for the [ to be verified, which needs to wait for the 
    { to be verified.

In the problem Remove All Adjacent Duplicates In String, some characters needed to wait for other characters to 
be deleted before they could be deleted themselves.

In the problem Backspace String Compare, a character could not be deleted until a character in front of it was 
deleted first.

We can identify the problem here as well. As mentioned in the explanation above, if you have 
temperatures = [34, 33, 32, ...], then we know that until we see a day that is warmer than 32, 
we definitely won't see a day that is warmer than 33 or 34. Thus we can process these days in reverse order.

When we find a temperature that is warmer than 32, we can start moving backward to see if it's warmer than 33 
or 34 as well.

Essentially, the stack holds temperatures that we have not yet found a warmer temperature for. Because we are 
forcing it to be monotonically decreasing, the temperature at the top of the stack will always be the coldest 
one.

As we iterate, for each temperature curr, we check if curr is warmer than the temperature at the top of the 
stack. Because we perform this check every iteration, if it is, then curr must be the first day warmer than the 
day at the top of the stack. We pop from the stack and continue checking the top until the stack is either empty 
or curr is no longer warmer. Maintaining the monotonic property conveniently has the side effect of processing 
all the answers.

Because the problem wants the number of days between the temperatures, we need to store the indices instead of 
the temperatures themselves. But this isn't an issue because given an index we can easily access the temperature 
from the input.

*/

/**
 * @param {number[]} temperatures
 * @return {number[]}
 */
var dailyTemperatures = function(temperatures) {
    let stack = [];
    let answer = new Array(temperatures.length).fill(0);
    
    for (let i = 0; i < temperatures.length; i++) {
        while (stack.length && temperatures[stack[stack.length - 1]] < temperatures[i]) {
            let j = stack.pop();
            answer[j] = i - j;
        }
        
        stack.push(i);
    }
    
    return answer;
};
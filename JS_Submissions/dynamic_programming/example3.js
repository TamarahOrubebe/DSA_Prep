/*
Example 3: 2140. Solving Questions With Brainpower

You are given a 0-indexed 2D integer array questions where questions[i] = [points_i, brainpower_i]
You have to process the questions in order. Solving question i will earn you points_i points but you will be 
unable to solve each of the next brainpower_i questions. If you skip question i, you get to decide on the next 
question. Return the maximum points you can score.


How can we tell this problem should be solved with DP? First, it is asking for a maximum score. Second, at every
question we need to make a decision: take or skip, and these decisions affect future decisions. If we decide to
take a question, it prevents us from taking some future questions.

As you may expect by now, we can define a function dp that returns the maximum score we can achieve. What 
information do we need at each state (other than an index variable i to indicate the current question we are on)?
We could include an integer that represents how many more questions we need to skip until we can start solving
questions again, but similar to with house robber, we can encode this information in our recurrence relation, 
so we'll just stick with dp(i) returning the maximum score.

If we are at the i^th question, we have two options:

Solve the question. We gain questions[i][0] points, but we cannot solve the next i questions. The next question 
we can solve is at index j = i + questions[i][1] + 1. Therefore, the total score is questions[i][0] + dp(j).
Skip the question. Like the problem says, this means we move on to the next question and make another decision 
there. The score is dp(i + 1).
This gives us our recurrence relation:

dp(i) = max(questions[i][0] + dp(j), dp(i + 1)), where j = i + questions[i][1] + 1

Since we are actually moving forward through the array instead of backward, this means the base case must be at 
the end. If i >= questions.length, that means the exam is over, so we can't score any more points. Therefore, 
the base case is:

dp(i) = 0, when i >= n
*/

/**
 * @param {number[][]} questions
 * @return {number}
 */
var mostPoints = function(questions) {
    let dp = i => {
        if (i >= questions.length) {
            return 0;
        }
        
        if (memo[i] != -1) {
            return memo[i];
        }
        
        let j = i + questions[i][1] + 1;
        memo[i] = Math.max(questions[i][0] + dp(j), dp(i + 1));
        return memo[i];
    }
    
    let memo = new Array(questions.length).fill(-1);
    return dp(0);
};

/* 
BOTTOM UP APPROACH

var mostPoints = function(questions) {
    let n = questions.length;
    let dp = new Array(n + 1).fill(0); // n + 1 to avoid out of bounds
    
    for (let i = n - 1; i >= 0; i--) {
        let j = i + questions[i][1] + 1;
        // need to make sure we don't go out of bounds
        dp[i] = Math.max(questions[i][0] + dp[Math.min(j, n)], dp[i + 1]);
    }
    
    return dp[0];
};

There are O(n) states, where nn is the length of the input array, and each state costs O(1) to compute, 
once again giving us a time and space complexity of O(n)O(n). Because the recurrence relation is not static
(it depends on questions[i][1]), we cannot improve the space complexity in the bottom-up implementation.

*/
/*
Coin Change

Solution
You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.

 

Example 1:

Input: coins = [1,2,5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1
Example 2:

Input: coins = [2], amount = 3
Output: -1
Example 3:

Input: coins = [1], amount = 0
Output: 0
 

Constraints:

1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104
*/

/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
    
    const dp = (rem) => {
        if (rem < 0) {
            return -1;
        }
        
        if (rem == 0) {
            return 0;
        }
        
        if(memo.has(rem)) {
            return memo.get(rem);
        }
        
        let minCost = Infinity;
        for(const coin of coins) {
            let res = dp(rem - coin);
            if(res != -1) {
                minCost = Math.min(minCost, res + 1);
            }
        }
        memo.set(rem, minCost != Infinity? minCost : -1)
       return memo.get(rem);
    }
    const memo = new Map();
    return dp(amount);
};
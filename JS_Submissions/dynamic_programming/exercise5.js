/*
Best Time to Buy and Sell Stock with Cooldown

Solution
You are given an array prices where prices[i] is the price of a given stock on the ith day.

Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and 
sell one share of the stock multiple times) with the following restrictions:

After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy 
again).

 

Example 1:

Input: prices = [1,2,3,0,2]
Output: 3
Explanation: transactions = [buy, sell, cooldown, buy, sell]
Example 2:

Input: prices = [1]
Output: 0
 

Constraints:

1 <= prices.length <= 5000
0 <= prices[i] <= 1000
*/

/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    const dp = (i, holding, coolDown) => {
        if (i == prices.length) {
            return 0;
        }
        
        
        if(memo[i][holding][coolDown] != -1) {
            return memo[i][holding][coolDown];
        }
        
        let ans = dp(i + 1, holding, coolDown);
        
        if(holding) {
            ans = Math.max(ans,  prices[i] + dp(i + 1, 0, 1));
        } else {
            if(coolDown) {
                ans = Math.max(ans, dp(i + 1, holding, 0));
            } else {
                 ans = Math.max(ans, -prices[i] + dp(i + 1, 1, coolDown));
            }
          
        }
       
        memo[i][holding][coolDown] = ans;
        return ans;
    }
    
    let memo = [];
    for (let i = 0; i < prices.length; i++) {
      memo.push([]);
       for (let j = 0; j < 2; j++) {
           memo[i].push(new Array(2).fill(-1));
       }
    }
    return dp(0, 0, 0);
};
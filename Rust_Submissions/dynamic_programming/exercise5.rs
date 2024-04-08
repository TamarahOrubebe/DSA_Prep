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


pub fn max_profit(prices: Vec<i32>) -> i32 {
         fn dp(prices: &Vec<i32>, i: usize, holding: bool, cool_down: bool, memo: &mut Vec<Vec<Vec<Option<i32>>>>) -> i32 {
            
            if i == prices.len() {
                return 0;
            }
            
            if let Some(val) = memo[i][holding as usize][cool_down as usize] {
                return val;
            }
            
            let mut ans = dp(prices, i + 1, holding, cool_down, memo);
            
            if holding {
                ans = ans.max(prices[i] + dp(prices, i + 1, false, true, memo));
            } else {
                if cool_down {
                    ans = ans.max(dp(prices, i + 1, holding, false, memo ));
                } else {
                     ans = ans.max(-prices[i] + dp(prices, i + 1, true, cool_down, memo));
                }
                
               
            }
            
            
            memo[i][holding as usize][cool_down as usize] = Some(ans);
            ans
            
            
        }
        
        let mut memo = vec![vec![vec![None; 2]; 2]; prices.len()];
        
        dp(&prices, 0, false, false, &mut memo)
}

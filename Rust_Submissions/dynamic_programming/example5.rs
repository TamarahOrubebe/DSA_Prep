/* 
Example 5: 188. Best Time to Buy and Sell Stock IV

You are given an integer array prices where prices[i] is the price of a given stock on the i^th day, and an 
integer k. You can buy the stock and sell it, but you can only hold on to one unit of stock at any given time. 
Find the maximum profit you can achieve with at most k transactions.

In this problem, there are lots of decisions to make. Should you buy today? If you do, you can't buy tomorrow, 
or at all until you sell. Should you sell? If you do, you can't sell until you buy again. Also, every time you 
buy or sell, you are using one of your limited number of transactions. With all these decisions to make, DP is a 
clear choice.

Again, our dp function should return the answer - so let's have it return the maximum profit that can be 
achieved. What information do we need at each state? First, we of course need to know what day it is. We can 
use our usual index variable i to track this. Next, we need to know if we are currently holding any stock. 
Let's have holding be a boolean that represents if we are holding stock. Lastly, the problem has an explicit 
numerical constraint - the number of transactions we are allowed. Let's have an integer remain that represents 
how many transactions we have remaining.

We will have dp(i, holding, remain) return the maximum profit we can achieve if we are currently on day i with 
remain transactions remaining, and holding indicating if we currently hold stock. The answer will be dp(0, 
false, k) which will return the maximum profit we can achieve starting on day 0, not holding stock, with k 
transactions remaining.

At each state, what decisions do we have? If we are not holding stock, we can either buy today or skip. If we 
buy, then our profit is -prices[i] + dp(i + 1, true, remain). We spend prices[i] to buy the stock. Then we move 
to the next day (i + 1), we are now holding stock (true), and we haven't completed a transaction yet so remain 
stays the same.

If we are holding stock, we can either sell today or skip. If we sell, then our profit is 
prices[i] + dp(i + 1, false, remain - 1). We gain prices[i] money. Then we move to the next day (i + 1), 
we are no longer holding stock (false), and we used up one of our transactions (remain - 1).

In both cases, if we decide to skip, then our profit is dp(i + 1, holding, remain). We just move to the next 
day without changing anything else. Therefore, our recurrence relation is:

dp(i, holding, remain) = max(skip, buy, sell) where

skip = dp(i + 1, holding, remain),

buy = -prices[i] + dp(i + 1, true, remain) and only considered if holding = false,

sell = prices[i] + dp(i + 1, false, remain - 1) and only considered if holding = true.

What are the base cases? If we reach the end of the input (i = prices.length), then we can't make any more 
transactions, so return 0. If we run out of transactions (k = 0), then we also cannot make any more transactions,
so return 0.

For holding, we can use 0 to represent false and 1 to represent true as well.
*/


fn max_profit(k: usize, prices: Vec<i32>) -> i32 {
    fn dp(i: usize, holding: bool, remain: usize, prices: &Vec<i32>, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i == prices.len() || remain == 0 {
            return 0;
        }
        
        if memo[i][holding as usize][remain] != -1 {
            return memo[i][holding as usize][remain];
        }
        
        let mut ans = dp(i + 1, holding, remain, prices, memo);
        if holding {
            ans = ans.max(prices[i] + dp(i + 1, false, remain - 1, prices, memo));
        } else {
            ans = ans.max(-prices[i] + dp(i + 1, true, remain, prices, memo));
        }
        
        memo[i][holding as usize][remain] = ans;
        ans
    }
    
    let mut memo = vec![vec![vec![-1; k + 1]; 2]; prices.len()];
    dp(0, false, k, &prices, &mut memo)
}

fn main() {
    let k = 2;
    let prices = vec![3,2,6,5,0,3];
    println!("{}", max_profit(k, prices));
}

/*
The work done at each state is O(1)O(1), so the time and space complexity is equal to the number of states. The
product of the range of each of the state variables is the number of states - which gives us a time and space 
complexity of O(n \cdot k)O(n⋅k). holding is constant, the n comes from i, and the k comes from remain.

*/
/*
Coin Change

Solution
You are given an integer array coins representing coins of different denominations and an integer amount 
representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made 
up by any combination of the coins, return -1.

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

use std::collections::HashMap;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
           fn dp(rem: i32, coins: &[i32], memo: &mut HashMap<i32, i32>) -> i32 {
            if rem < 0 {
                return -1;
            }

            if rem == 0 {
                return 0;
            }

            if let Some(&val) = memo.get(&rem) {
                return val;
            }

            let mut min_cost = i32::MAX;
            for coin in coins {
                let res = dp(rem - coin, coins, memo);
                if res != -1 {
                    min_cost = min_cost.min(res + 1);
                }
            }
            let value = if min_cost != i32::MAX {
                min_cost
            } else {
                -1
            };
            memo.insert(rem, value);
            value
        }

        
        let mut memo: HashMap<i32, i32> = HashMap::new();
        dp(amount, &coins, &mut memo)
}

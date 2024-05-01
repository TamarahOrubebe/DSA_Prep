/*
Example 6: 2218. Maximum Value of K Coins From Piles

There are n piles of coins on a table. Each pile consists of a positive number of coins of assorted denominations
. In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet. Given a list 
piles, where piles[i] is a list of integers denoting the composition of the i^th pile from top to bottom, and a 
positive integer k, return the maximum total value of coins you can have in your wallet if you choose exactly k 
coins optimally.

You know the drill by now - we'll use a function dp that returns the most money we can take. We have our usual 
index variable i that can represent the current pile we are on, and the problem also has the explicit numerical 
constraint k, so we'll use another state variable remain that represents how many more coins we can take. So we 
have dp(i, remain) which returns the maximum value of coins we can take starting from the i^th pile with remain 
moves remaining.

At the i^th pile, we can either skip the pile or take some coins. If we skip, then the score is dp(i + 1, remain)
. If we don't skip, we can choose how many coins to take. If we take coins up to the j^th coin, then the score 
will be equal to the sum of piles[i][:j] plus dp(i + 1, remain - j - 1). We just need to make sure that we don't 
take more than remain coins.

At every state, we need to try all possibilities. In a given state, we can use an integer variable curr to track
the value of the coins we have taken from the current pile, then iterate over the pile and find the maximum 
score possible. The recurrence relation is:

dp(i, remain) = max(skip, take), where

skip = dp(i + 1, remain), and

take = max( sum(piles[i][:j]) + dp(i + 1, remain - j - 1) for j from 0 to min(remain, piles[i].length) )

This looks scary, but it becomes simpler when you look at each term on its own. sum(piles[i][:j]) is the value 
of the coins we have taken at the current pile, j + 1 is the number of coins we have taken, so that leaves us 
with remain - j - 1 moves, and min(remain, piles[i].length) is just to make sure that we don't take more coins 
than we are allowed to.

What are our base cases? If we either reach the end of the input (no piles left) or remain = 0 (can't take any 
more coins), then the maximum score we can achieve is 0.
*/

fn max_value_of_coins(piles: Vec<Vec<i32>>, k: usize) -> i32 {
    fn dp(piles: &Vec<Vec<i32>>, i: usize, remain: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == piles.len() || remain == 0 {
            return 0;
        }
        
        if memo[i][remain] != -1 {
            return memo[i][remain];
        }
        
        let mut ans = dp(&piles, i + 1, remain, memo); // Skip this pile
        let mut curr = 0;
        for j in 0..std::cmp::min(remain, piles[i].len()) {
            curr += piles[i][j];
            ans = ans.max(curr + dp(&piles, i + 1, remain - j - 1, memo));
        }
        
        memo[i][remain] = ans;
        ans
    }
    
    let mut memo = vec![vec![-1; k + 1]; piles.len()];
    dp(&piles, 0, k, &mut memo)
}

fn main() {
    // Test example
    let piles = vec![vec![2, 4, 1], vec![2, 3, 10], vec![1, 5, 3], vec![4, 2, 1]];
    let k = 8;
    let max_value = max_value_of_coins(piles, k);
    println!("Maximum value of coins: {}", max_value);
}

/*
Let's say that the average number of coins per pile piles[i].length is x. There are O(n⋅k) states, 
where n is the number of piles, and at each state, we have a for loop that iterates x times. This gives us a 
time complexity of O(n⋅k⋅x) and a space complexity of O(n⋅k).

*/
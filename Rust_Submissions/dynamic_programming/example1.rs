/*
Example 1: 198. House Robber

You are planning to rob houses along a street. The i^th house has nums[i] money. If you rob two houses beside 
each other, the alarm system will trigger and alert the police. What is the most money you can rob without 
alerting the police?

First things first: what will our function return and what arguments will it take? The return value should be the 
maximum money that can be robbed since that is what the problem is asking for. What about state variables? We 
definitely need i - the maximum money that can be robbed if we only consider up to and including house i. Since 
we aren't allowed to rob the previous house, should we also include a boolean prev that indicates if we robbed 
the previous house? We certainly could include that - but it's not necessary.

We can consider the case of having robbed the previous house in our recurrence relation. At house i, what 
choices do we have? There are two possibilities:

We rob the house. This means we gain nums[i] money, but we cannot rob the previous house. If we need to skip the
previous house, that means we must have arrived from 2 houses back. The amount of money we had 2 houses ago is 
dp(i - 2). Therefore, if we rob the i^th house, we will have dp(i - 2) + nums[i] money.

We don't rob the house. This means we don't gain any money, but we could have arrived from the previous house, 
which means we will have dp(i - 1) money.
We should always choose the maximum profit. Therefore, our recurrence relation is 
dp(i) = max(dp(i - 1), dp(i - 2) + nums[i]).

What are the base cases? If there is only one house, we may as well rob it. If there are two houses, we can 
only rob one, so we should rob the one with more money. The base cases are:

dp(0) = nums[0] and dp(1) = max(nums[0], nums[1])

We need two base cases because the recurrence on dp(1) would need dp(-1) if we didn't handle dp(1) as a base 
case.
*/

fn rob(nums: Vec<i32>) -> i32 {
    let mut memo = vec![-1; nums.len()];

    fn dp(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
        // Base cases
        if i == 0 {
            return nums[0];
        }

        if i == 1 {
            return nums[0].max(nums[1]);
        }

        if memo[i] != -1 {
            return memo[i];
        }

        // Recurrence relation
        memo[i] = (dp(i - 1, nums, memo)).max(dp(i - 2, nums, memo) + nums[i]);
        memo[i]
    }

    if nums.len() == 1 {
        // to prevent out of bounds error
        return nums[0];
    }

    dp(nums.len() - 1, &nums, &mut memo)
}

fn main() {
    let nums = vec![2, 7, 9, 3, 1];
    println!("{}", rob(nums));
}
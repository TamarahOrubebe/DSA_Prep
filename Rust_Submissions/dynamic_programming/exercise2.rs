/*
Min Cost Climbing Stairs

Solution
You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost,
you can either climb one or two steps.

You can either start from the step with index 0, or the step with index 1.

Return the minimum cost to reach the top of the floor.

 

Example 1:

Input: cost = [10,15,20]
Output: 15
Explanation: You will start at index 1.
- Pay 15 and climb two steps to reach the top.
The total cost is 15.
Example 2:

Input: cost = [1,100,1,1,1,100,1,1,100,1]
Output: 6
Explanation: You will start at index 0.
- Pay 1 and climb two steps to reach index 2.
- Pay 1 and climb two steps to reach index 4.
- Pay 1 and climb two steps to reach index 6.
- Pay 1 and climb one step to reach index 7.
- Pay 1 and climb two steps to reach index 9.
- Pay 1 and climb one step to reach the top.
The total cost is 6.
 

Constraints:

2 <= cost.length <= 1000
0 <= cost[i] <= 999
*/

fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        fn dp(cost: &Vec<i32>, i: usize, memo: &mut Vec<i32>) -> i32 {
            if i <= 1 {
                return 0;
            }
            
            if memo[i] != - 1 {
                return memo[i];
            }

            memo[i] = i32::min(
                dp(cost, i - 1, memo) + cost[i - 1],
                dp(cost, i - 2, memo) + cost[i - 2]);
            memo[i]
        }
        
        let mut memo = vec![-1; cost.len() + 1];
        dp(&cost, cost.len(), &mut memo)
}
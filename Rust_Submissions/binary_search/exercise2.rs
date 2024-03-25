/*
 Longest Subsequence With Limited Sum

Solution
You are given an integer array nums of length n, and an integer array queries of length m.

Return an array answer of length m where answer[i] is the maximum size of a subsequence that you can take from 
nums such that the sum of its elements is less than or equal to queries[i].

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

 

Example 1:

Input: nums = [4,5,2,1], queries = [3,10,21]
Output: [2,3,4]
Explanation: We answer the queries as follows:
- The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of such a
subsequence, so answer[0] = 2.
- The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size of 
such a subsequence, so answer[1] = 3.
- The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size of 
such a subsequence, so answer[2] = 4.

Example 2:

Input: nums = [2,3,4,5], queries = [1]
Output: [0]
Explanation: The empty subsequence is the only subsequence that has a sum less than or equal to 1, so 
answer[0] = 0.
 

Constraints:

n == nums.length
m == queries.length
1 <= n, m <= 1000
1 <= nums[i], queries[i] <= 106
*/

pub fn answer_queries(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut ans = vec![];
        
//         for i in 1..nums.len() {
//             nums[i] += nums[i - 1 as usize];
//         }
        
//         fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
//             let mut left = 0;
//             let mut right = arr.len() as i32 - 1;
            
//             while left <= right {
//                 let mid = (left + right) / 2;
                
//                 if arr[mid as usize] > target {
//                     right = mid - 1;
//                 } else {
//                     left = mid + 1;
//                 }
//             }
//             left
//         }
        
//         for query in queries {
//             let idx = binary_search(nums.clone(), query);
//             ans.push(idx);
//         }
        for query in queries.iter_mut() {
            let mut count = 0;
            for num in &nums {
                  if *query >= *num {
                      *query -= *num;
                      count += 1;
                } else {
                    break;
                }
            }
            ans.push(count);
        }
        ans
}

/*
Numbers With Same Consecutive Differences

Solution
Given two integers n and k, return an array of all the integers of length n where the difference between every 
two consecutive digits is k. You may return the answer in any order.

Note that the integers should not have leading zeros. Integers as 02 and 043 are not allowed.

 

Example 1:

Input: n = 3, k = 7
Output: [181,292,707,818,929]
Explanation: Note that 070 is not a valid number, because it has leading zeroes.
Example 2:

Input: n = 2, k = 1
Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
 

Constraints:

2 <= n <= 9
0 <= k <= 9
*/

use std::collections::HashSet;
pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        fn backtrack(ans: &mut Vec<i32>, num: i32, n: i32, k: i32) {
            if n == 0 {
                ans.push(num);
                return;
            }
            
            let last_digit = num % 10;
            let new_digits = HashSet::from([last_digit + k, last_digit - k]);
            for new_digit in new_digits {
                if 0 <= new_digit && new_digit <= 9 {
                    let new_num = num * 10 + new_digit;
                    backtrack(ans, new_num, n - 1, k);
                }
            }
        }
        
        let mut ans = vec![];
        for i in (1..=9) {
             backtrack(&mut ans, i, n - 1, k);
        }
       ans
    }
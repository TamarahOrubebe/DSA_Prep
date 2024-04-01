/* 
Generate Parentheses

Solution
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

 

Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
Example 2:

Input: n = 1
Output: ["()"]
 

Constraints:

1 <= n <= 8
*/

pub fn generate_parenthesis(n: i32) -> Vec<String> {
        
        fn backtrack(n: i32, ans: &mut Vec<String>, curr:&mut String, left_total: i32, right_total: i32) {
            if curr.len() as i32 == n * 2 {
                ans.push(curr.clone());
                return;
            }
            
            if left_total < n {
                curr.push('(');
                backtrack(n, ans, curr, left_total + 1, right_total);
                curr.pop();
            }
            
            if left_total > right_total {
                curr.push(')');
                backtrack(n, ans, curr, left_total, right_total + 1);
                curr.pop();
            }
            
        }
        
        let mut ans = Vec::new();
        let mut curr = String::new();
        backtrack(n, &mut ans, &mut curr, 0, 0);
        ans
    }
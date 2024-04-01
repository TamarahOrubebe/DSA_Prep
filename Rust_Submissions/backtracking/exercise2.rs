/*
Letter Combinations of a Phone Number

Solution
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number 
could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to 
any letters.


 

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
Example 2:

Input: digits = ""
Output: []
Example 3:

Input: digits = "2"
Output: ["a","b","c"]
 

Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].
*/

use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
            if digits.len() as i32 == 0 {
                return Vec::new();
            }
            
        let mut hash: HashMap<char, &str> = HashMap::new();
        hash.insert('2', "abc");
        hash.insert('3', "def");
        hash.insert('4', "ghi");
        hash.insert('5', "jkl");
        hash.insert('6', "mno");
        hash.insert('7', "pqrs");
        hash.insert('8', "tuv");
        hash.insert('9', "wxyz");
        
            fn backtrack(digits: &String, curr: &mut String, idx: usize, hash: &HashMap<char, &str>, ans: &mut Vec<String>) {
                if idx == digits.len() {
                    ans.push(curr.clone());
                    return;
                }
                
                let letters = hash.get(&digits.chars().nth(idx).unwrap()).unwrap();
                
                for char in letters.chars() {
                    curr.push(char);
                    backtrack(&digits, curr, idx + 1 as usize, hash, ans);
                    curr.pop();
                }
        }
        
        let mut curr = String::new();
        let mut ans = Vec::new();
        backtrack(&digits, &mut curr, 0, &hash, &mut ans);
        ans
    }
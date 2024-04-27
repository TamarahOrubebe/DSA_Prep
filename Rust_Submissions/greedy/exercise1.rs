/*
Maximum 69 Number

Solution
You are given a positive integer num consisting only of digits 6 and 9.

Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).

 

Example 1:

Input: num = 9669
Output: 9969
Explanation: 
Changing the first digit results in 6669.
Changing the second digit results in 9969.
Changing the third digit results in 9699.
Changing the fourth digit results in 9666.
The maximum number is 9969.
Example 2:

Input: num = 9996
Output: 9999
Explanation: Changing the last digit 6 to 9 results in the maximum number.
Example 3:

Input: num = 9999
Output: 9999
Explanation: It is better not to apply any change.
 

Constraints:

1 <= num <= 104
num consists of only 6 and 9 digits.
*/

use std::collections::BinaryHeap;
pub fn maximum69_number (num: i32) -> i32 {
        let num_string = num.to_string();
        let mut heap: BinaryHeap<String> = BinaryHeap::new();
        heap.push(num_string.clone());

        for (i, char) in num_string.chars().enumerate() {
            if char == '9' {
                heap.push(format!("{}6{}", &num_string[..i], &num_string[i + 1..]));
            } else {
                heap.push(format!("{}9{}", &num_string[..i], &num_string[i + 1..]));
            }
            
        }
        
        heap.pop().unwrap().parse().unwrap()
}

// time complexity O(n.logn) space complexity O(n) because of the heap where n is the length of the digit.

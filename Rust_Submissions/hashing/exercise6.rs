/*
Maximum Number of Balloons

Solution
Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

You can use each character in text at most once. Return the maximum number of instances that can be formed.

 

Example 1:



Input: text = "nlaebolko"
Output: 1
Example 2:



Input: text = "loonbalxballpoon"
Output: 2
Example 3:

Input: text = "leetcode"
Output: 0
 

Constraints:

1 <= text.length <= 104
text consists of lower case English letters only.
*/

fn max_number_of_balloons(text: String) -> i32 {
        let mut b_count = 0;
        let mut a_count = 0;
        let mut l_count = 0;
        let mut o_count = 0;
        let mut n_count = 0;
        
        for char in text.chars() {
           match char {
               'b' => b_count += 1,
               'a' => a_count += 1,
               'l' => l_count += 1,
               'o' => o_count += 1,
               'n' => n_count += 1,
               _ => ()
            }
        }
        
        l_count /= 2;
        o_count /= 2;
        
       i32::min(i32::min(i32::min(b_count, a_count), i32::min(l_count, o_count)), n_count)
    }

    // time complexity O(n), space complexity O(1)
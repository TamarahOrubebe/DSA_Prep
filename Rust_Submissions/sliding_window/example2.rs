/*
Example 2: You are given a binary string s (a string containing only "0" and "1"). You may choose up to one "0" 
and flip it to a "1". What is the length of the longest substring achievable that contains only "1"?

For example, given s = "1101100111", the answer is 5. If you perform the flip at index 2, the string becomes 
1111100111.
*/

pub fn find_length(s: &str) -> i32 {
    let (mut left, mut curr, mut ans) = (0, 0, 0);
    let s_chars: Vec<char> = s.chars().collect();
    for right in 0..s_chars.len() {
        if s_chars[right] == '0' {
            curr += 1;
        }
        
        while curr > 1 {
            if s_chars[left] == '0' {
                curr -= 1;
            }
            left += 1;
        }
        
        ans = ans.max(right - left + 1);
    }
    
    ans as i32
}

/* 
this problem runs in O(n) time, where nn is the length of s, as the work done in each loop iteration is 
amortized constant. Only a few integer variables are used as well, which means this algorithm uses O(1) space.
*/
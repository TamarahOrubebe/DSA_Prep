/* 
Example 1: 1143. Longest Common Subsequence

Given two strings text1 and text2, return the length of their longest common subsequence.

For example, given text1 = "abcde" and text2 = "ace", return 3. Both strings share "ace" as a subsequence.

How do we know this problem should be solved with DP? First, it's asking for the longest of something. Second, 
deciding to use a letter or not use a letter affects the future letters we can take.

As the problem is asking for the length of the longest common subsequence (LCS), let's have our function dp 
return the length of the LCS. We need two index variables this time, one for each string - i for text1 and j for
text2. We'll have dp(i, j) return the length of the longest common subsequence when we start at the i^th character
of text1 and the j^th character of text2.

At each pair (i, j) there are two possibilities:

text1[i] = text2[j]. We found a match in characters and should use it to increase the length. After matching the
characters, we need to move to the next character in both strings. dp(i, j) = 1 + dp(i + 1, j + 1). There is no
point in not using a match because we can't increase our length by more than 1 at any given step, so when we 
have the opportunity to, we should always take it.

text1[i] != text2[j]. Now, we need to make a decision. Either we can move to the next character in text1 or move
to the next character in text2. We may as well try both - so in this case, 
dp(i, j) = max(dp(i + 1, j), dp(i, j + 1)).

These 2 cases form our recurrence relation. What about base cases? If i = text1.length or j = text2.length, 
then one of the strings has been exhausted, and since no characters are remaining, there cannot be any common
characters. Return 0.

*/
fn longest_common_subsequence(text1: &str, text2: &str) -> usize {
    fn dp(i: usize, j: usize, text1: &Vec<char>, text2: &Vec<char>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == text1.len() || j == text2.len() {
            return 0;
        }
        
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        
        if text1[i] == text2[j] {
            memo[i][j] = 1 + dp(i + 1, j + 1, text1, text2, memo);
            return memo[i][j];
        }
        
        memo[i][j] = (dp(i + 1, j, text1, text2, memo)).max(dp(i, j + 1, text1, text2, memo));
        memo[i][j]
    }
    
    let text1_chars: Vec<char> = text1.chars().collect();
    let text2_chars: Vec<char> = text2.chars().collect();
    let mut memo = vec![vec![-1; text2.len()]; text1.len()];
    dp(0, 0, &text1_chars, &text2_chars, &mut memo) as usize
}

fn main() {
    let text1 = "abcde";
    let text2 = "ace";
    println!("{}", longest_common_subsequence(text1, text2));
}

// Because the work done at each state is O(1) this algorithm has a time and space complexity of 
// O(n⋅m), where n = text1.length and m = text2.length.
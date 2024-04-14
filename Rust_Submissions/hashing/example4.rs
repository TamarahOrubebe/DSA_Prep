/*
Example 1: You are given a string s and an integer k. Find the length of the longest substring that contains at 
most k distinct characters.

For example, given s = "eceba" and k = 2, return 3. The longest substring with at most 2 distinct characters is 
"ece".
*/

use std::collections::HashMap;

fn find_longest_substring(s: &str, k: usize) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut ans = 0;

    for (right, ch) in s.chars().enumerate() {
        *counts.entry(ch).or_insert(0) += 1;

        while counts.len() > k {
            if let Some(count) = counts.get_mut(&s.chars().nth(left).unwrap()) {
                *count -= 1;
                if *count == 0 {
                    counts.remove(&s.chars().nth(left).unwrap());
                }
            }
            left += 1;
        }

        ans = ans.max(right - left + 1);
    }

    ans
}

fn main() {
    let s = String::from("eceba");
    let k = 2;
    println!("{}", find_longest_substring(&s, k)); // Output: 3
}

/*
As you can see, using a hash map to store the frequency of any key we want allows us to solve sliding window 
problems that put constraints on multiple elements. We know from earlier that the time complexity of sliding 
window problems are O(n) if the work done inside each for loop iteration is amortized constant, which is the 
case here due to a hash map having O(1) operations. The hash map occupies O(k) space, as the algorithm will 
delete elements from the hash map once it grows beyond k.
*/
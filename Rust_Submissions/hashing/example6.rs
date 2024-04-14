/*
Example 3: 1941. Check if All Characters Have Equal Number of Occurrences

Given a string s, determine if all characters have the same frequency.

For example, given s = "abacbc", return true. All characters appear twice. Given s = "aaabb", return false. "a" 
appears 3 times, "b" appears 2 times. 3 != 2.
*/

use std::collections::{HashMap, HashSet};


fn are_occurrences_equal(s: &str) -> bool {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let frequencies: HashSet<_> = counts.values().cloned().collect();

    frequencies.len() == 1
}
fn main() {

    println!("{}", are_occurrences_equal("abacbc")); // Output: true
}

/*
Given nn as the length of s, it costs O(n) to populate the hash map, then O(n) to convert the hash map's 
values to a set. This gives us a time complexity of O(n). The space that the hash map and set would occupy 
is equal to the number of unique characters. As previously discussed, some people would argue that this is O(1)
since the characters come from the English alphabet, which is bounded by a constant. A more general answer 
would be to say that the space complexity is O(k), where kk is the number of characters that could be in the
input, which happens to be 26 in this problem.
*/
/*
Example 1: 49. Group Anagrams

Given an array of strings strs, group the anagrams together.

For example, given strs = ["eat","tea","tan","ate","nat","bat"], return [["bat"],["nat","tan"],["ate","eat","tea"]].
*/

use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let key = {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            chars.into_iter().collect()
        };

        groups.entry(key).or_insert(Vec::new()).push(s);
    }

    groups.into_iter().map(|(_, v)| v).collect()
}
fn main() {
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    println!("{:#?}", group_anagrams(strs)); // Output: 
}

/*
Given nn as the length of strs and mm as the average length of the strings, we iterate over each string and sort 
it, which costs O(n⋅m⋅logm). Then, we need to iterate over the keys. In the worst case 
scenario, when there are no matching anagrams, there will be nn groups, which means this will cost O(n), 
giving an overall time complexity of O(n⋅m⋅logm) (the final + n+n is dominated). The 
space complexity is O(n⋅m) as each string will be placed in an array within the hash map.
*/
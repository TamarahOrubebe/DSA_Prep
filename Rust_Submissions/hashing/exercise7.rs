/* 
Jewels and Stones

Solution
You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so "a" is considered a different type of stone from "A".

 

Example 1:

Input: jewels = "aA", stones = "aAAbbbb"
Output: 3
Example 2:

Input: jewels = "z", stones = "ZZ"
Output: 0
 

Constraints:

1 <= jewels.length, stones.length <= 50
jewels and stones consist of only English letters.
All the characters of jewels are unique.
*/


fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut hash = HashMap::new();
        
        for char in stones.chars() {
            *hash.entry(char).or_insert(0) += 1;
        }
        
        let mut ans = 0;
        for char in jewels.chars() {
            if hash.contains_key(&char) {
                ans += hash.get(&char).unwrap()
            }
        }
    ans
}

// time complexity O(m + n), space complexity O(n).
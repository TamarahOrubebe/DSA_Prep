/*
Example 5: 1248. Count Number of Nice Subarrays

Given an array of positive integers nums and an integer k. Find the number of subarrays with exactly k odd 
numbers in them.

For example, given nums = [1, 1, 2, 1, 1], k = 3, the answer is 2. The subarrays with 3 odd numbers in them are 
[1, 1, 2, 1, 1] and [1, 1, 2, 1, 1].

*/

use std::collections::HashMap;

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut counts = HashMap::new();
    counts.insert(0, 1);
    let mut ans = 0;
    let mut curr = 0;

    for num in nums {
        curr += num % 2;
        ans += counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}
fn main() {
    let nums = vec![1, 1, 2, 1, 1];
    println!("{}", number_of_subarrays(nums, 3)); // Output: 4
}

/*
The time and space complexity of this algorithm is identical to the previous problem's (O(n) for both) for 
the same reasons. Remember when we said, "you'll see how similar the code is for each problem that falls in this 
pattern"? Two different problems and the difference in the code is literally 2 characters, "% 2". Note that 
while not all problems that follow this pattern will use identical code, they will all still be very similar.
*/
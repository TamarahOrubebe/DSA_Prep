/*
Example 4: 560. Subarray Sum Equals K

Given an integer array nums and an integer k, find the number of subarrays whose sum is equal to k.
*/

use std::collections::HashMap;

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut counts = HashMap::new();
    counts.insert(0, 1);
    let mut ans = 0;
    let mut curr = 0;

    for num in nums {
        curr += num;
        ans += counts.get(&(curr - k)).unwrap_or(&0);
        println!("{}", ans);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}
fn main() {
    let nums = vec![1, 2, 1, 2, 1];
    println!("{}", subarray_sum(nums, 3)); // Output: 4
}

/*
To summarize:

We use curr to track the prefix sum.
At any index i, the sum up to i is curr. If there is an index j whose prefix is curr - k, then the sum of the 
subarray with elements from j + 1 to i is curr - (curr - k) = k.
Because the array can have negative numbers, the same prefix can occur multiple times. We use a hash map counts 
to track how many times a prefix has occurred.
At every index i, the frequency of curr - k is equal to the number of subarrays whose sum is equal to k that end
at i. Add it to the answer.
The time and space complexity of this algorithm are both O(n), where nn is the length of nums. Each for loop
iteration runs in constant time and the hash map can grow to a size of nn elements.


*/
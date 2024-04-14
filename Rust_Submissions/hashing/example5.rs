/*
Example 2: 2248. Intersection of Multiple Arrays

Given a 2D array nums that contains n arrays of distinct integers, return a sorted array containing all the numbers that appear 
in all n arrays.

For example, given nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]], return [3, 4]. 3 and 4 are the only numbers that 
are in all arrays.
*/

use std::collections::HashMap;

fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut counts = HashMap::new();

    // Count occurrences of each number
    for arr in &nums {
        for x in arr {
            *counts.entry(x).or_insert(0) += 1;
        }
    }

    let n = nums.len();
    let mut ans = Vec::new();

    // Check for numbers occurring in all arrays
    for (&key, &val) in &counts {
        if val == n {
            ans.push(*key);
        }
    }

    ans.sort(); // Sort the result
    ans
}
fn main() {
        let nums = vec![
        vec![3,1,2,4,5],
        vec![1,2,3,4],
        vec![3,4,5,6],
    ];

    println!("{:#?}", intersection(nums)); // Output: [3, 4]
}

/*
This problem is a good discussion point for why a hash map is convenient. You may be thinking, since our keys 
are integers, why can't we just use an array instead of a hash map? We could, but the problem is that the array 
needs to be at least as large as the maximum element. What if we have a test case like [1, 2, 3, 1000]? We need 
to initialize an array of size 1000, even though only a few of the indices will actually be used. Therefore, 
using an array could end up being a huge waste of space. Sure, sometimes it would be more efficient because of 
the overhead of a hash map, but overall, a hash map is much safer. Even if 99999999999 is in the input, it 
doesn't matter - the hash map handles it like any other element.

Let's say that there are nn lists and each list has an average of mm elements. To populate our hash map, it 
costs O(n⋅m) to iterate over all the elements. The next loop iterates over all unique elements that 
we encountered. If all elements are unique, this can cost up to O(n⋅m), although this won't affect 
our time complexity since the previous loop also cost O(n⋅m). Finally, there can be at most m 
elements inside ans when we perform the sort, which means in the worst case, the sort will cost O(m⋅logm). This 
gives us a time complexity of O(n⋅m + m⋅logm) = O(m⋅(n+logm))
.If every element in the input is unique, then the hash map will grow to a size of n⋅m, which means the 
algorithm has a space complexity of O(n⋅m).
*/
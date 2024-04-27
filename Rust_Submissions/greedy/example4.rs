/*
Example 4: 1481. Least Number of Unique Integers after K Removals

Given an array of integers arr and an integer k, find the least number of unique integers after removing exactly
k elements.
*/

/*
Example 4: 1481. Least Number of Unique Integers after K Removals

Given an array of integers arr and an integer k, find the least number of unique integers after removing exactly k elements.
*/
use std::collections::HashMap;

fn find_least_num_of_unque_ints(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    let mut ans = Vec::new();
    for num in nums {
        *hash.entry(num).or_insert(0) += 1;
    }
    
    for (_key, value) in hash {
        ans.push(value);
    }
    
    ans.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    while k > 0 {
        if let Some(&num) = ans.last() {
            if num <= k {
                ans.pop();
                k -= num;
            } else {
                break;
            }
        }
    }
    ans.len() as i32
}
fn main() {
    let elements = vec![4, 3, 1, 1, 3, 3, 2];
    println!("{:#?}", find_least_num_of_unque_ints(elements, 3));
}

/*
In the worst case scenario where all elements are unique, there will be n keys in our hash map, where n is the 
length of the input array, so the sort will cost O(n⋅logn). Each iteration in our while loop runs in O(1), and it
can also run at most nn times, giving a final time complexity of O(n⋅logn). The space complexity is O(n) due to 
the hash map.

*/
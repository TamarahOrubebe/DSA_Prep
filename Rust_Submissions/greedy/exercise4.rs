/*
Reduce Array Size to The Half

Solution
You are given an integer array arr. You can choose a set of integers and remove all the occurrences of these 
integers in the array.

Return the minimum size of the set so that at least half of the integers of the array are removed.

 

Example 1:

Input: arr = [3,3,3,3,5,5,5,2,2,7]
Output: 2
Explanation: Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size 
of the old array).
Possible sets of size 2 are {3,5},{3,2},{5,2}.
Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has a size greater than 
half of the size of the old array.
Example 2:

Input: arr = [7,7,7,7,7,7]
Output: 1
Explanation: The only possible set you can choose is {7}. This will make the new array empty.
 

Constraints:

2 <= arr.length <= 105
arr.length is even.
1 <= arr[i] <= 105
*/

use std::collections::{HashMap, BinaryHeap};


    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let target = arr.len() / 2;
        let mut hash: HashMap<i32, i32> = HashMap::new();
        
        for num in arr {
            *hash.entry(num).or_insert(0) += 1;
        }
        let mut heap: BinaryHeap<(i32, i32)> = hash.into_iter().map(|(num, freq)| (freq, num)).collect();
         
        let mut removed = 0;
        let mut ans = 0;
        
        while removed < target {
            let (freq, _) = heap.pop().unwrap();
            removed += freq as usize;
            ans += 1;
        }
       ans
}

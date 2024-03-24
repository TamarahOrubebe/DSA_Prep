/*
Example 4: 1481. Least Number of Unique Integers after K Removals

Given an array of integers arr and an integer k, find the least number of unique integers after removing exactly
k elements.
*/

/**
 * @param {number[]} arr
 * @param {number} k
 * @return {number}
 */
var findLeastNumOfUniqueInts = function(arr, k) {
    let counts = new Map();
    for (const num of arr) {
        counts.set(num, (counts.get(num) || 0) + 1);
    }
    
    let ordered = [];
    for (const val of counts.values()) {
        ordered.push(val);
    }
    
    ordered.sort((a, b) => b - a);
    while (k > 0) {
        let val = ordered[ordered.length - 1];
        if (val <= k) {
            k -= val;
            ordered.pop();
        } else {
            break;
        }
    }
    
    return ordered.length;
};

/*
In the worst case scenario where all elements are unique, there will be n keys in our hash map, where n is the 
length of the input array, so the sort will cost O(n \cdot \log{}n)O(n⋅logn). Each iteration in our while loop 
runs in O(1)O(1), and it can also run at most nn times, giving a final time complexity of O(n⋅logn). The space 
complexity is O(n) due to the hash map.

*/
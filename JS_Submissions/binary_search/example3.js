/*
Example 3: 2300. Successful Pairs of Spells and Potions

You are given two positive integer arrays spells and potions, where spells[i] represents the strength of the 
i^th spell and potions[j] represents the strength of the j^th potion. You are also given an integer success. 
A spell and potion pair is considered successful if the product of their strengths is at least success. For each
spell, find how many potions it can pair with to be successful. Return an integer array where the i^th element 
is the answer for the i^th spell.

Let's say we sort potions and have potions = [1, 2, 3, 4, 5], and success = 7.

We have a spell with a strength of 3. To form a successful pair, we need a potion with a strength of at least 
7 / 3 = 2.3333.

If we do a binary search for this value on potions, we will find an insertion index of 2. Every potion on this 
index and to the right can form a successful pair. There are 3 indices in total (the potions with strength 3, 4,
 5).

In general, if there are m potions, the final index is m - 1. If the insertion index is i, then the range
[i, m - 1] has a size of (m - 1) - i + 1 = m - i.

We can iterate over the spells, and for each spell, perform a binary search on success / spell to find the 
insertion index i, then use the formula to find the number of potions that can form a successful pair.
*/

/**
 * @param {number[]} spells
 * @param {number[]} potions
 * @param {number} success
 * @return {number[]}
 */
var successfulPairs = function(spells, potions, success) {
    let binarySearch = (arr, target) => {
        left = 0;
        right = arr.length - 1;
        while (left <= right) {
            let mid = Math.floor((left + right) / 2);
            if (arr[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        return left;
    }
    
    potions.sort((a, b) => a - b);
    let ans = [];
    let m = potions.length;
    
    for (const spell of spells) {
        let i = binarySearch(potions, success / spell);
        ans.push(m - i);
    }
    
    return ans;
};

/*
o sort potions, it costs O(m⋅logm). Then, we iterate nn times, performing a O(logm) 
binary search on each iteration. This gives us a time complexity of O((m+n)⋅logm), which is much faster than 
O(m⋅n) because mlogm is small. Because we are sorting the input, some space is used depending on the sorting 
algorithm used by your language.

*/
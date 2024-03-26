/*
Example 1: 875. Koko Eating Bananas

Koko loves to eat bananas. There are n piles of bananas, the i^{th}i 
th
  pile has piles[i] bananas. Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses a 
  pile and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them and will not 
  eat any more bananas during the hour. Return the minimum integer k such that she can eat all the bananas 
  within h hours.

  There are multiple steps to identifying the usage of binary search and then the implementation.

First, the problem is asking for the minimum eating speed. If Koko can eat all the bananas with a speed of k, 
she can surely eat all the bananas with a speed of k + 1 as well. If she cannot, she surely cannot with a speed 
of k - 1 either.

This creates the two "zones". Next, we need to identify the search space bounds. No matter what the input is, we 
know the eating speed can never be lower than 1 because the eating speed must be an integer, and an eating speed 
of 0 would mean no bananas ever get eaten. If the eating speed is max(piles), then each pile will take one hour. 
It is impossible to improve on this (since the problem states that even if the eating speed is greater than the 
    number of bananas in a pile, it will still take one hour). Therefore, our search space is [1, max(piles)].

Finally, we need to figure out how to implement the check function for a given eating speed k.

The problem statement says that if a pile has less than k bananas, Koko will eat them all and then not eat 
anymore for the rest of the hour. If a pile has 8 bananas and Koko has an eating speed of 3, then Koko can eat 
6 bananas in 2 hours, leaving 2 bananas left in the pile. Koko then needs to spend another hour eating the 
remaining 2 bananas.

The number of hours required to eat a pile of bananas with an eating speed of k is bananas / k, rounded up. We 
can iterate over the input, sum the values of bananas / k, and check if it is within the limit.

With the search space identified and check function implemented, we can perform a binary search to find the 
threshold.
*/

/**
 * @param {number[]} piles
 * @param {number} h
 * @return {number}
 */
var minEatingSpeed = function(piles, h) {
    let check = k => {
        let hours = 0;
        for (const bananas of piles) {
            hours += Math.ceil(bananas / k);
        }
        
        return hours <= h;
    }
    
    let left = 1;
    let right = Math.max(...piles);
    while (left <= right) {
        let mid = Math.floor((left + right) / 2);
        if (check(mid)) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    
    return left;
};

/* 
check runs in O(n)O(n) time, where n = piles.length. The binary search runs in O(\log{}k)O(logk) time, where 
k = max(piles). This gives us a time complexity of O(n⋅logk), which is very fast since log 
is an extremely fast function. We don't use any extra space except for a few integer variables.


*/

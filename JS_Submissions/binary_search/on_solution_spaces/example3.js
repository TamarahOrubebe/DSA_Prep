/*
Example 3: 1870. Minimum Speed to Arrive on Time

You are given a float hour, representing the amount of time you have to reach the office. To commute to the 
office, you must take n trains in sequential order. You are also given an integer array dist, where dist[i] 
describes the distance of the i^th train ride. Each train can only depart at an integer hour, so you may need to
wait in between each train ride.

For example, if the 1st train ride takes 1.5 hours, you must wait for an additional 0.5 hours before you can 
depart on the 2nd train ride at the 2-hour mark. Return the minimum positive integer speed that all the trains 
must travel at for you to reach the office on time, or -1 if it is impossible to be on time. The answer will not
exceed 10^7


This problem is fairly similar to Koko Eating Bananas. If we have a speed k, and a train has a distance d, then 
the train takes d / k time. The problem has an added constraint that trains only depart on integer hours - so we 
also need to round our time up before taking each train.

The minimum possible speed is 1, and the maximum is 10^7 as given in the problem description. The fact that the 
problem is giving us this information is actually a hint towards using binary search, but it brings up a good 
question - what do you do when you cannot ascertain a maximum possible answer from what is given in the input? 
You can use an arbitrarily large number for right, like 10^10. Logarithms are so fast that it will hardly make a
difference.

The task can also just be impossible in general - the problem says we should return -1 if it is. When is it 
impossible? Because each train only departs on an integer, even if we have a ridiculously high speed, each train
will take at least 1 hour. Therefore, if there are more trains than hours allowed, it is impossible.

Note: in the check function, we don't take the ceiling of the final value because we don't need to wait for any 
more trains. The rounding up is to simulate waiting for the next train at the next integer hour.
*/

/**
 * @param {number[]} dist
 * @param {number} hour
 * @return {number}
 */
var minSpeedOnTime = function(dist, hour) {
    let check = k => {
        let t = 0;
        for (const d of dist) {
            t = Math.ceil(t);
            t += d / k;
        }
        
        return t <= hour;
    }
    
    if (dist.length > Math.ceil(hour)) {
        return -1;
    }
    
    let left = 1;
    let right = Math.pow(10, 7);
    
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
Technically, this problem has a time complexity of O(n)O(n), where nn is the length of the input array, because 
the problem explicitly tells us that the maximum answer is 10^7 and the logarithm of this is a constant. However,
it would be more appropriate to say that the algorithm has a time complexity of O(n⋅logk), where
kk is the maximum possible answer. We don't use any extra space except for a few integer variables.


*/
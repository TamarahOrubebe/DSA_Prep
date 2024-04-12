/*
Example 1: Given an array of positive integers nums and an integer k, find the length of the longest subarray 
whose sum is less than or equal to k. 
*/

pub fn find_length(nums: &[i32], k: i32) -> i32 {
    let (mut left, mut curr, mut ans) = (0, 0, 0);
    for right in 0..nums.len() {
        curr += nums[right];
        while curr > k {
            curr -= nums[left];
            left += 1;
        }
        ans = ans.max((right - left + 1) as i32);
    }
    ans
}

/*
You may be thinking: there is a while loop inside of the for loop, isn't the time complexity O(n^2)? The reason 
it is still O(n) is that the while loop can only iterate nn times in total for the entire algorithm 
(left starts at 0, only increases, and never exceeds n). If the while loop were to run n times on one iteration 
of the for loop, that would mean it wouldn't run at all for all the other iterations of the for loop. This is 
what we refer to as amortized analysis - even though the worst case for an iteration inside the for loop is O(n),
it averages out to O(1) when you consider the entire runtime of the algorithm.

The space complexity is constant because we are only using 3 integer variables.
*/
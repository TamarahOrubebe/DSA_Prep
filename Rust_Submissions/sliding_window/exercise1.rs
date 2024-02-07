/*
Maximum Average Subarray I

Solution
You are given an integer array nums consisting of n elements, and an integer k.

Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.

 

Example 1:

Input: nums = [1,12,-5,-6,50,3], k = 4
Output: 12.75000
Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
Example 2:

Input: nums = [5], k = 1
Output: 5.00000
 

Constraints:

n == nums.length
1 <= k <= n <= 105
-104 <= nums[i] <= 104

*/
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut left: usize = 0;
    let mut w_len = 0;
    let mut curr = 0;
    let mut ans: f64 = 0.0;
    let len = nums.len() as i32;

        
    for i in 0..k {
        curr += nums[i as usize];
        w_len += 1;
    }
        
    ans = curr as f64 / k as f64; 
        
    for right in k..len {
        curr += nums[right as usize];
        w_len += 1;
            
        while w_len > k {
            curr -= nums[left];
            w_len -= 1;
            left += 1;
        }
            
        ans = f64::max(ans, curr as f64 / k as f64);
    }
    ans
}

// the work done in each loop iteration is amortized constant, so this algorithm has a runtime of O(n),
// where nn is the length of nums, and O(1) space.

// O(k) + amortised O(n - k) == O(n)
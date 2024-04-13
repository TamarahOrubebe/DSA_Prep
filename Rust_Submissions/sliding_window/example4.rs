/*
Example 4: Given an integer array nums and an integer k, find the sum of the subarray with the largest sum whose
length is k.


*/

pub fn find_best_sub_array (nums: Vec<i32>, k: i32) -> i32 {
    let (mut sum, mut ans) = (0, 0);
    let new_k = k as usize;
    for i in 0..new_k {
        sum += nums[i];
    }
    ans = sum;
    for i in new_k..nums.len() {
        sum += nums[i] - nums[i - k];
        ans = ans.max(sum);
    }

    ans
}

// The total for loop iterations is equal to n, where nn is the length of nums, and the work done in each 
// iteration is constant, giving this algorithm a time complexity of O(n), using O(1) space.
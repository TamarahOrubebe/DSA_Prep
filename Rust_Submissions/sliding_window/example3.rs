/*
Example 3: 713. Subarray Product Less Than K.

Given an array of positive integers nums and an integer k, return the number of subarrays where the product of 
all the elements in the subarray is strictly less than k.

For example, given the input nums = [10, 5, 2, 6], k = 100, the answer is 8. The subarrays with products less 
than k are:

[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
*/

pb fn  num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let (mut left, mut ans) = (0, 0);
    let mut curr_prod = 1;

    for right in 0..nums.len() {
        curr *= nums[right];
        while curr >= k {
            curr /= nums[left];
            left += 1;
        }

        ans += (right - left + 1) as i32;
    }

    ans
}

// Again, the work done in each loop iteration is amortized constant, so this algorithm has a runtime of O(n), 
// where nn is the length of nums, and O(1) space.




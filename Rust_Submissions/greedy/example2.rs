/*
Example 2: 2294. Partition Array Such That Maximum Difference Is K

Given an integer array nums and an integer k, split nums into subsequences, where each subsequences' maximum and 
minimum element is within k of each other. What is the minimum number of subsequences needed?

For example, given nums = [3, 6, 1, 2, 5] and k = 2, the answer is 2. The subsequences are [3, 1, 2] and [6, 5].
*/

fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let mut x = nums[0];
    let mut ans = 1;
    for i in 1..nums.len() {
        if nums[i] - x > k {
            x = nums[i];
            ans += 1;
        }
    }
    ans
}
fn main() {
    let elements = vec![3, 6, 1, 2, 5];
    println!("{:#?}", partition_array(elements, 2));
}

/*
This runs in O(n⋅logn) where nn is the length of the input array due to the sort. Again, the only extra space 
used is during the sort, and the complexity depends on the sorting algorithm used by your language of choice.
*/
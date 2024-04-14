/*
Example 1: 1. Two Sum

Given an array of integers nums and an integer target, return indices of two numbers such that they add up to 
target. You cannot use the same index twice.
*/

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![i as i32, j as i32];
        }
        
        map.insert(num, i);
    }

    vec![-1, -1]
}

/*
If the question wanted us to return a boolean indicating if a pair exists or to return the numbers themselves, 
then we could just use a set. However, since it wants the indices of the numbers, we need to use a hash map to 
"remember" what indices the numbers are at.

The time complexity is O(n) as the hash map operations are O(1). This solution also uses O(n) space 
as the number of keys the hash map will store scales linearly with the input size.


*/
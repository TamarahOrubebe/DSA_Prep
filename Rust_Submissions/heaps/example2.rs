/*Example 2: 2208. Minimum Operations to Halve Array Sum

You are given an array nums of positive integers. In one operation, you can choose any number from nums and 
reduce it to exactly half the number.
Return the minimum number of operations to reduce the sum of nums by at least half.
*/ 

use std::collections::BinaryHeap;
fn last_stone_weight(nums: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
        let mut target = heap.iter().sum(); 

        target /= 2;
        let mut num_of_operations = 0;
        while target > 0 {
            if let Some(num) = heap.pop() {
                target -= num / 2;
                heap.push(num / 2);
                num_of_operations += 1;
            }
        }
      
      num_of_operations
}

fn main() {
    let stones = vec![5, 19, 8, 1];
    println!("{:#?}", last_stone_weight(stones));
}

/*
Each iteration of the loop takes O(logn) time from the heap operations. The number of operations 
needed is linear with n. While you may be thinking: if we have a huge number, it would need to be halved many 
times. True, but each operation on it would also reduce the sum by a large amount. This gives us a time 
complexity of O(n⋅logn).
*/
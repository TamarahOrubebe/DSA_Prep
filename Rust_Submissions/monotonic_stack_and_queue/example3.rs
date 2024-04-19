/*
Example 3: 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

Given an array of integers nums and an integer limit, return the size of the longest subarray such that the 
absolute difference between any two elements of this subarray is less than or equal to limit.
*/

/*
Example 3: 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

Given an array of integers nums and an integer limit, return the size of the longest subarray such that the 
absolute difference between any two elements of this subarray is less than or equal to limit.
*/
use std::collections::VecDeque;

fn longest_subarray(nums: Vec<i32>, limit:i32) -> i32 {
   let mut increasing: VecDeque<i32> = VecDeque::new();
   let mut decreasing: VecDeque<i32> = VecDeque::new();
   let mut left = 0;
   let mut ans = 0;
   
   for (right, &num) in nums.iter().enumerate() {
       while !increasing.is_empty() && *increasing.back().unwrap() > num {
           increasing.pop_back();
       } 
       
       while !decreasing.is_empty() && *decreasing.back().unwrap() < num {
           decreasing.pop_back();
       }
       
       increasing.push_back(num);
       decreasing.push_back(num);
       
       while decreasing.front().unwrap() - increasing.front().unwrap() > limit {
                if nums[left as usize] == *increasing.front().unwrap() {
                    increasing.pop_front();
                }

                if nums[left as usize] == *decreasing.front().unwrap() {
                    decreasing.pop_front();
                }

                left += 1;
        }     
        
       
       ans = ans.max(right - left + 1);
   }
   
   ans as i32
}

fn main() {
    let nums = vec![10, 1, 2, 4, 7, 2];
    let limit = 5;
    println!("The length of the longest continuous subarray with max difference <= 5 is: {:#?}", longest_subarray(nums, limit));
    // return 4
}

/*
With efficient queues, this algorithm has a time and space complexity of O(n) as each for loop iteration is 
amortized O(1) and the deques can grow to size n, where nn is the size of nums.
*/
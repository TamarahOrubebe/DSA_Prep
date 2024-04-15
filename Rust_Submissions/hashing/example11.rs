/*
Example 3: 2342. Max Sum of a Pair With Equal Sum of Digits

Given an array of integers nums, find the maximum value of nums[i] + nums[j], where nums[i] and nums[j] have the 
same digit sum (the sum of their individual digits). Return -1 if there is no pair of numbers with the same digit
sum.


Recall in the group anagrams problem, a group could have many different strings in it so we needed an easy way to
identify which strings belonged to which group. We used sorted strings as an "identifier" for each string.

In this problem, many numbers could have the same digit sum. We can simply use the digit sum as an identifier for
each number. For each digit sum, we can just take the two maximum elements in the group and find their sum.
*/


use std::collections::HashMap;

fn maximum_sum(nums: Vec<i32>) -> i32 {
    // Function to calculate the digit sum of a number
    fn get_digit_sum(mut num: i32) -> i32 {
        let mut digit_sum = 0;
        while num > 0 {
            digit_sum += num % 10;
            num /= 10;
        }
        digit_sum
    }

    let mut dic: HashMap<i32, Vec<i32>> = HashMap::new();

    // Populate the HashMap with digit sum as key and corresponding numbers as values
    for num in nums {
        let digit_sum = get_digit_sum(num);
        dic.entry(digit_sum).or_insert(Vec::new()).push(num);
    }

    let mut ans = -1;

    // Iterate over the HashMap entries to find the maximum sum of two numbers with the same digit sum
    for (_, curr) in &dic {
        if curr.len() > 1 {
            let mut sorted_curr = curr.clone();
            sorted_curr.sort_by(|a, b| b.cmp(a));
            ans = ans.max(sorted_curr[0] + sorted_curr[1]);
        }
    }

    ans
}

fn main() {
    let nums = vec![51, 71, 17, 42];
    println!("{}", maximum_sum(nums)); // Output: 93
}

/*
This algorithm is inefficient due to the sorting, which can potentially cost O(n⋅logn) if every number in the 
input has the same digit sum, where nn is the length of the input array. Just like in the previous problem, we 
don't need to store all the numbers in the group. We can improve the time complexity and average space complexity
by only saving the largest number seen so far for each digit sum.
*/
fn maximum_sum(nums: Vec<i32>) -> i32 {
    // Function to calculate the digit sum of a number
    fn get_digit_sum(mut num: i32) -> i32 {
        let mut digit_sum = 0;
        while num > 0 {
            digit_sum += num % 10;
            num /= 10;
        }
        digit_sum
    }

    let mut dic: HashMap<i32, i32> = HashMap::new();
    let mut ans = -1;

    // Populate the HashMap with digit sum as key and corresponding numbers as values
    for num in nums {
        let digit_sum = get_digit_sum(num);
        
       if let Some(&max_num) = dic.get(&digit_sum) {
            // Update ans if the current number plus the maximum number for the same digit sum is greater
            ans = ans.max(max_num + num);
            // Update the maximum number for the digit sum if the current number is larger
            dic.insert(digit_sum, max_num.max(num));
        } else {
            // If there's no maximum number for the digit sum yet, insert the current number
            dic.insert(digit_sum, num);
        }
    }

    ans
}


/*
the first algorithm always uses O(n) space because we store all the elements in the hash map's values, but 
with the improvement, the average case will use much less space since each key only stores an integer. We also 
save on an extra iteration and a sort in each iteration, giving us a time complexity of O(n), where nn is 
the length of the input array.
*/
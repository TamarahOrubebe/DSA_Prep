/*
Example 1: 739. Daily Temperatures

Given an array of integers temperatures that represents the daily temperatures, return an array answer such that 
answer[i] is the number of days you have to wait after the i^th day to get a warmer temperature. If there is no 
future day that is warmer, have answer[i] = 0 instead.

*/

/* 
Example 1: 739. Daily Temperatures

Given an array of integers temperatures that represents the daily temperatures, return an array answer such that answer[i] is the number of days
you have to wait after the i^th day to get a warmer temperature. If there is no future day that is warmer, have answer[i] = 0 instead.
*/
fn get_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0;temperatures.len()];
    let mut stack: Vec<usize> = Vec::new();
    
    for (i, &temperature) in temperatures.iter().enumerate() {
        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperature {
            let j = stack.pop().unwrap();
            
            ans[j] = (i - j) as i32; 
        }
        
        stack.push(i);
    }
    
    ans
}

fn main() {
    let temperatures = vec![40, 35, 32, 37, 50];
    println!("The warmer days for each temperature is: {:#?}", get_temperatures(temperatures));
}

/*
As we discussed earlier in the sliding window chapter, despite the nested loop, the time complexity is still O(n),
where n is the length of the array, because the inner while loop can only iterate over each element once across 
all for loop iterations, making the for loop iterations amortized O(1).

the space complexity is O(n) for the stack. 
*/
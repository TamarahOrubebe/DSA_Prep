/*
Example 3: Given two sorted integer arrays arr1 and arr2, return a new array that combines both of them and is 
also sorted.
*/

pub fn combine(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            ans.push(arr1[i]);
            i += 1;
        } else {
            ans.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        ans.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        ans.push(arr2[j]);
        j += 1;
    }

    ans
}

// this algorithm has a time complexity of O(n) and uses O(1) space (if we don't count the output as 
// extra space, which we usually don't).


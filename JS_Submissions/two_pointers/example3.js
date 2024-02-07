/*
Example 3: Given two sorted integer arrays arr1 and arr2, return a new array that combines both of them and
is also sorted.

*/

/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @return {number[]}
 */
let combine = function(arr1, arr2) {
    // ans is the answer
    let ans = [];
    let i = 0, j = 0;
    
    while (i < arr1.length && j < arr2.length) {
        if (arr1[i] < arr2[j]) {
            ans.push(arr1[i]);
            i++;
        } else {
            ans.push(arr2[j]);
            j++;
        }
    }
    
    while (i < arr1.length) {
        ans.push(arr1[i]);
        i++;
    }
    
    while (j < arr2.length) {
        ans.push(arr2[j]);
        j++;
    }
    
    return ans;
}

    // this algorithm has a time complexity of O(n) and uses O(1) space 
    // (if we don't count the output as extra space, which we usually don't).
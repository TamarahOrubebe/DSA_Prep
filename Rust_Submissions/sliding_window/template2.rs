/*
Fixed window size
In the examples we looked at above, our window size was dynamic. We tried to expand it to the right as much as 
we could while keeping the window within some constraint and removed elements from the left when the constraint 
was violated. Sometimes, a problem will specify a fixed length k.

These problems are easy because the difference between any two adjacent windows is only two elements 
(we add one element on the right and remove one element on the left to maintain the length).

Start by building the first window (from index 0 to k - 1). Once we have a window of size k, if we add an 
element at index i, we need to remove the element at index i - k. For example, k = 2 and you currently have 
elements at indices [0, 1]. Now, we add 2: [0, 1, 2]. To keep the window size at k = 2, 
we need to remove 2 - k = 0: [1, 2].

Here's some pseudocode:

function fn(arr, k):
    curr = some data to track the window

    // build the first window
    for (int i = 0; i < k; i++)
        Do something with curr or other variables to build first window

    ans = answer variable, probably equal to curr here depending on the problem
    for (int i = k; i < arr.length; i++)
        Add arr[i] to window
        Remove arr[i - k] from window
        Update ans

    return ans

*/

fn main() {
    println!("Hello Rustaceans!");
}
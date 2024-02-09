/*
Another way to use two pointers
This method where we start the pointers at the first and last indices and move them towards each other is only one way to implement two pointers. Algorithms are beautiful because of how abstract they are - "two pointers" is just an idea, and it can be implemented in many different ways. Let's look at another method and some new examples. The following method is applicable when the problem has two iterables in the input, for example, two arrays.

Move along both inputs simultaneously until all elements have been checked.

Converting this idea into instructions:

Create two pointers, one for each iterable. Each pointer should start at the first index.
Use a while loop until one of the pointers reaches the end of its iterable.
At each iteration of the loop, move the pointers forward. This means incrementing either one of the pointers or both of the pointers. Deciding which pointers to move will depend on the problem we are trying to solve.
Because our while loop will stop when one of the pointers reaches the end, the other pointer will not be at the end of its respective iterable when the loop finishes. Sometimes, we need to iterate through all elements - if this is the case, you will need to write extra code here to make sure both iterables are exhausted.
Here's some pseudocode illustrating the concept:

function fn(arr1, arr2):
    i = j = 0
    while i < arr1.length AND j < arr2.length:
        Do some logic here depending on the problem
        Do some more logic here to decide on one of the following:
            1. i++
            2. j++
            3. Both i++ and j++

    // Step 4: make sure both iterables are exhausted
    // Note that only one of these loops would run
    while i < arr1.length:
        Do some logic here depending on the problem
        i++

    while j < arr2.length:
        Do some logic here depending on the problem
        j++
*/

fn main() {
    println!("Hello Rustaceans!");
}
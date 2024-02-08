/*
When should we use sliding window?
There is a very common group of problems involving subarrays that can be solved efficiently with sliding window. 
Let's talk about how to identify these problems.

First, the problem will either explicitly or implicitly define criteria that make a subarray "valid". 
There are 2 components regarding what makes a subarray valid:

A constraint metric. This is some attribute of a subarray. It could be the sum, the number of unique elements, 
the frequency of a specific element, or any other attribute.
A numeric restriction on the constraint metric. This is what the constraint metric should be for a subarray to 
be considered valid.

For example, let's say a problem declares a subarray is valid if it has a sum less than or equal to 10. 
The constraint metric here is the sum of the subarray, and the numeric restriction is <= 10. A subarray is 
considered valid if its constraint metric conforms to the numeric restriction, i.e. the sum is less than or 
equal to 10.

Second, the problem will ask you to find valid subarrays in some way.

The most common task you will see is finding the best valid subarray. The problem will define what makes a 
subarray better than another. For example, a problem might ask you to find the longest valid subarray.

Another common task is finding the number of valid subarrays. We will take a look at this later in the article.

Whenever a problem description talks about subarrays, you should figure out if sliding window is a good option 
by analyzing the problem description. If you can find the things mentioned above, then it's a good bet.


Pseud-code template:

function fn(arr):
    left = 0
    for (int right = 0; right < arr.length; right++):
        Do some logic to "add" element at arr[right] to window

        while WINDOW_IS_INVALID:
            Do some logic to "remove" element at arr[left] from window
            left++

        Do some logic to update the answer
*/
/*
Example 5: 881. Boats to Save People

You are given an array people where people[i] is the weight of the i^th person. A boat can hold up to two people,
if their weight combined is less than or equal to limit. What is the fewest number of boats you need to carry
everyone? Note: no person is heavier than limit.
*/

fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    
    people.sort();
    let mut i = 0;
    let mut j = people.len() - 1;
    
    while i <= j {
        if people[i] + people[j] <= limit {
            i += 1;
        }
        
        ans += 1;
        j -= 1;
    }
    ans
}
fn main() {
    let people = vec![3, 2, 2, 1];
    println!("{:#?}", num_rescue_boats(people, 3));
}

/*
Every iteration of the while loop will put the current heaviest person in a boat (that's why we do j-- on every 
iteration). If the lightest person can also come along, then we include them as well (i++).

Like with the two pointer implementations we have already seen, the two pointers part of the algorithm runs in 
O(n), where nn = people.length. However, we needed to sort the input for this algorithm to work, which gives us 
a time complexity of O(n⋅logn). The only extra space used is for the sort, which as mentioned previously will 
depend on the language you're using.

As we can see from the examples, most greedy problems involve sorting the input. You could argue that a lot of 
the problems we have looked at in earlier chapters were greedy problems - especially in the previous chapter 
(heap). Most greedy problems just require some logical reasoning to see that a greedy approach works, and then 
implementation is relatively straightforward. 
*/
/* 
Example 2: 2260. Minimum Consecutive Cards to Pick Up

Given an integer array cards, find the length of the shortest subarray that contains at least one duplicate. If 
the array has no duplicates, return -1.



We can actually solve this problem using a sliding window, but let's take a look at another approach that has 
more emphasis on a hash map. This question is equivalent to: what is the shortest distance between any two of 
the same element? If we go through the array and use a hash map to record the indices for every element, we can 
iterate over those indices to find the shortest distance. For example, given cards = [1, 2, 6, 2, 1], we would 
map 1: [0, 4], 2: [1, 3], and 6: [2]. Then we can iterate over the values and see that the minimum difference 
can be achieved from picking up the 2s.
*/

use std::collections::HashMap;

fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, &card) in cards.iter().enumerate() {
        dic.entry(card).or_insert(Vec::new()).push(i);
    }

    let mut ans = std::i32::MAX;
    for (_, arr) in dic.iter() {
        for i in 0..arr.len() - 1 {
            ans = ans.min((arr[i + 1] - arr[i] + 1) as i32);
        }
    }

    if ans == std::i32::MAX {
        -1
    } else {
        ans
    }
}
fn main() {
    let cards = vec![1, 2, 6, 2, 1];
    println!("{:#?}", minimum_card_pickup(cards)); // Output: 
}

/* 
The time complexity is still O(n) even though we have a nested loop in the algorithm. This is because the inner 
loop in the nested loop can only iterate nn times in total, since it's iterating over indices of elements from 
the array, where nn is the length of the input array.

We can actually improve this algorithm slightly by observing that we don't need to store all the indices, but 
only the most recent one that we saw for each number. This improves the average space complexity. The current 
algorithm has O(n) space complexity always, but with the improvement, it is only O(n) in the worst case, when 
there are no duplicates.

*/


fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, i32> = HashMap::new();
    let mut ans = i32::MAX;
    for (i, &card) in cards.iter().enumerate() {
        if dic.contains_key(&card) {
            ans = ans.min(i as i32 - *dic.get(&card).unwrap() + 1);
        }
        
        dic.insert(card, i as i32);
    }

    if ans == std::i32::MAX {
        -1
    } else {
        ans
    }
}


/*
This algorithm also runs faster because we save on an iteration, although the time complexity of both algorithms
is O(n), where nn is the length of the input array.


*/
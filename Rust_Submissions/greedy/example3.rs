/*Example 3: 502. IPO

LeetCode would like to work on some projects to increase its capital before IPO. You are given n projects where the i^th project has a profit of
profits[i] and a minimum capital of capital[i] is needed to start it. Initially, you have w capital. When you finish a project, the profit will
be added to your total capital. Return the max capital possible if you are allowed to do up to k projects.
*/

use std::collections::BinaryHeap;
use std::iter::zip;

fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let n = profits.len() as i32;
    let mut projects: Vec<_> = zip(capital, profits).collect();
    projects.sort();
    let mut i = 0;
    
    for _ in 0..k {
        while i < n && projects[i as usize].0 <= w {
            heap.push(projects[i as usize].1);
            i += 1;
        }
        
        if heap.is_empty() {
            return w;
        }
        
        w += heap.pop().unwrap();
    }
    w
}


fn main() {
    let profits = vec![3, 1, 6, 12, 20];
    let capital = vec![0, 0, 2, 5, 7];
    println!("The maximized capital is {}", find_maximized_capital(3, 1, profits, capital));
}

/*
This algorithm has a time complexity of O((k+n)⋅logn), where n is the number of projects given. The heap's max 
size is n, which means its operations are \log{}nlogn in the worst case, and we do k + n operations 
(k pop operations, n push operations). The sort at the start also costs O(n⋅logn), but this 
doesn't change the complexity. The space complexity is O(n) due to the heap.


*/
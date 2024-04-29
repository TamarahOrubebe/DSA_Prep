/*
Example 3: 77. Combinations

Given two integers n and k, return all combinations of k numbers out of the range [1, n] in any order.

For example, given n = 4, k = 2, return [[2,4],[3,4],[2,3],[1,2],[1,3],[1,4]].


*/
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn backtrack(curr: &mut Vec<i32>, i: i32, n: i32, k: i32, ans: &mut Vec<Vec<i32>>) {
        if curr.len() == k as usize {
            ans.push(curr.clone());
            return;
        }
        
        for num in i..=n {
            curr.push(num);
            backtrack(curr, num + 1, n, k, ans);
            curr.pop();
        }
    }
    
    let mut ans = Vec::new();
    backtrack(&mut Vec::new(), 1, n, k, &mut ans);
    ans
}

fn main() {
    let n = 4;
    let k = 2;
    println!("{:?}", combine(n, k));
}

/*
The time complexity of this algorithm is very difficult to derive. It is O({k . n!}/{(n - k)! . k!})In an 
interview, it's understandable if you can't find the exact time complexity, but you should still
do your best.

A good strategy is to find the upper bound on the time complexity. On the first call, the for loop runs n times.
The next call can run n - 1 times, then the next n - 2 and so forth, which leads to O(n!). However, the max depth
is k - which means the factorial's multiplication doesn't go down to 1, it goes down to n - k, so we need to 
divide our factorial by (n - k)!.

We also need to copy each combination which costs O(k) - this gives us O({k . n!}/{(n - k)!})
​
 ), which is a good approximation of the actual time complexity. Any interviewer should be satisfied with an 
 nswer like this, provided you explain your thought process.

The space complexity is O(k)O(k) for curr and the recursion call stack.

As you can see, the idea and code behind each of these problems is very similar. Remember to model the problem 
as a tree, and then figure out what children each node should have. Try these upcoming practice problems on your 
own before moving on.


*/
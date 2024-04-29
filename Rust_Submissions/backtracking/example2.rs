/*
Example 2: 78. Subsets

Given an integer array nums of unique elements, return all subsets in any order without duplicates.

For example, given nums = [1, 2, 3], return [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*/

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &[i32], start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            backtrack(nums, i + 1, path, ans);
            path.pop();
        }
    }

    let mut ans = Vec::new();
    backtrack(&nums, 0, &mut Vec::new(), &mut ans);
    ans
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", subsets(nums));
}

/*
This time our base case is when i > nums.length - we have run out of numbers to use. The modification of curr 
and the undoing of the modification remain the same.

You may notice that the base case is never actually hit because the function can't be called with an argument 
greater than the length of the input. We have included the condition for clarity.

There are 2^n subsets, where nn is the length of the input array - for each element, we can either take it or not take it. 
For the time complexity, you can think of the algorithm as a DFS on a tree with 2^{n} nodes. At each node, we 
make a copy of curr, so the time complexity is O(n . 2^n). The space complexity is O(n) for curr and the 
recursion call stack.
 */
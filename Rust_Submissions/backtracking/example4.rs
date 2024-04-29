/*
Example 1: 39. Combination Sum

Given an array of distinct positive integers candidates and a target integer target, return a list of all unique
combinations of candidates where the chosen numbers sum to target. The same number may be chosen from candidates
an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers
is different.
*/

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        curr: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if curr == target {
            ans.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            let num = candidates[i];
            if curr + num <= target {
                path.push(num);
                backtrack(candidates, target, i, curr + num, path, ans);
                path.pop();
            }
        }
    }

    let mut ans = Vec::new();
    backtrack(&candidates, target, 0, 0, &mut Vec::new(), &mut ans);
    ans
}

fn main() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    println!("{:?}", combination_sum(candidates, target));
}

/*
The time complexity of this algorithm is approximately O(n^T/M), where n = candidates.length, T = target, and M =
min(candidates). Recall that recursion in general can be thought of as a tree. The maximum depth of the tree in 
this problem is T/M using the smallest number repeatedly until we exceed target. Each node in the tree can have 
up to n children, which gives us O(n^T/M)

If not counting the output as extra space, the space used in this problem is for path, and the recursion call 
stack, both of which are O(T/M).
*/
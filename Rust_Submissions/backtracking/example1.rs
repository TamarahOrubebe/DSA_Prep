/*
Example 1: 46. Permutations

Given an array nums of distinct integers, return all the possible permutations in any order.

For example, given nums = [1, 2, 3], return [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]].
*/

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &[i32], curr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if curr.len() == nums.len() {
            ans.push(curr.clone());
            return;
        }

        for &num in nums {
            if !curr.contains(&num) {
                curr.push(num);
                backtrack(nums, curr, ans);
                curr.pop();
            }
        }
    }

    let mut ans = Vec::new();
    backtrack(&nums, &mut Vec::new(), &mut ans);
    ans
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", permute(nums));
}


/*
When adding to the answer, we need to create a copy of curr because curr is only a reference to the array's 
address.

The time complexity of this algorithm is very slow, but the input says that 1 <= nums.length <= 6, so it is to 
be expected. Let n = nums.length. The initial call to backtrack (the "root" of the tree), makes n calls. Each of 
those calls makes n - 1 calls (avoiding duplicates), and each of those makes n - 2, and so on. As such, we can 
expect approximately O(n!) calls. In the function, we have a loop over the nn input elements, and in each 
iteration, we check if num is already in curr, which costs O(n). Thus, we can estimate each function call to
cost about O(n^2). Note that we could optimize this process by keeping track of elements in curr using a 
separate hash set, allowing us to perform the checks in O(1).

Now, using this logic, we could say that the time complexity is O(n^2 . n!), or O(n . n!) if we implemented the 
optimization. However, this is not the true time complexity of the algorithm. The true time complexity is 
extremely complicated and requires higher-level mathematics. It is typical for backtracking problems to have 
very difficult to calculate time complexities - but as long as you present the logic like we have here, you will
be fine in an interview.
*/
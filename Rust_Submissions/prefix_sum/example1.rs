/*
Example 1: Given an integer array nums, an array queries where queries[i] = [x, y] and an integer limit, return 
a boolean array that represents the answer to each query. A query is true if the sum of the subarray from x to y
is less than limit, or false otherwise.

For example, given nums = [1, 6, 3, 2, 7, 2], queries = [[0, 3], [2, 5], [2, 4]], and limit = 13, the answer is 
[true, false, true]. For each query, the subarray sums are [12, 14, 12].
*/

pub fn answer_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>, limit: i32) -> Vec<bool> {
    let mut prefix = vec![nums[0]];

    for i in 1..nums.len() {
        prefix.push(prefix[prefix.len() - 1] + nums[i]);
    }
    let mut ans = Vec::new();
    for query in queries {
        let x = query[0] as usize;
        let y = query[1] as usize;

        let curr = prefix[y] - prefix[x - 1];
        ans.push(curr < limit);
    }

    ans
}

/*
Without the prefix sum, answering each query would be O(n) in the worst case, where n is the length of nums.
If m = queries.length, that would give a time complexity of O(n * m). With the prefix sum, it costs O(n) to build,
but then answering each query is O(1). This gives a much better time complexity of O(n + m). We use O(n) space to
build the prefix sum.
*/
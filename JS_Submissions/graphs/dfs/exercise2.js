/*
Number of Connected Components in an Undirected Graph

Solution
You have a graph of n nodes. You are given an integer n and an array edges where edges[i] = [ai, bi] indicates that there is an edge between ai and bi in the graph.

Return the number of connected components in the graph.

 

Example 1:


Input: n = 5, edges = [[0,1],[1,2],[3,4]]
Output: 2
Example 2:


Input: n = 5, edges = [[0,1],[1,2],[2,3],[3,4]]
Output: 1
 

Constraints:

1 <= n <= 2000
1 <= edges.length <= 5000
edges[i].length == 2
0 <= ai <= bi < n
ai != bi
There are no repeated edges.
*/

/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */
var countComponents = function(n, edges) {
    
    let graph = new Map();
    let ans = 0;
    let dfs = (node) => {
        for(let neighbor of graph.get(node)) {
            if(!seen[neighbor]) {
                seen[neighbor] = true;
                dfs(neighbor);
            }
        }
    }
    
    for (let i = 0; i < n; i++) {
        graph.set(i, []);
    }
    
    for (const [x, y] of edges) {
        graph.get(x).push(y);
        graph.get(y).push(x);
    }
    
    let seen = new Array(n).fill(false);
    
    for (let i = 0; i < n; i++) {
        if(seen[i] == false) {
            ans++;
            dfs(i);
        }
    }
    
    return ans;
    
};

// time and space complexities of O(n);
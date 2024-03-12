/*
Reachable Nodes With Restrictions

Solution
There is an undirected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

You are given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree. You are also given an integer array restricted which represents restricted nodes.

Return the maximum number of nodes you can reach from node 0 without visiting a restricted node.

Note that node 0 will not be a restricted node.

 

Example 1:


Input: n = 7, edges = [[0,1],[1,2],[3,1],[4,0],[0,5],[5,6]], restricted = [4,5]
Output: 4
Explanation: The diagram above shows the tree.
We have that [0,1,2,3] are the only nodes that can be reached from node 0 without visiting a restricted node.
Example 2:


Input: n = 7, edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]], restricted = [4,2,1]
Output: 3
Explanation: The diagram above shows the tree.
We have that [0,5,6] are the only nodes that can be reached from node 0 without visiting a restricted node.
 

Constraints:

2 <= n <= 105
edges.length == n - 1
edges[i].length == 2
0 <= ai, bi < n
ai != bi
edges represents a valid tree.
1 <= restricted.length < n
1 <= restricted[i] < n
All the values of restricted are unique.
*/


use std::collections::{HashMap, HashSet};
fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        
        fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut HashSet<i32>, ans: &mut i32) -> i32 {
            
            *ans += 1;
            
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if !seen.contains(neighbor) {
                        seen.insert(*neighbor);
                        dfs(*neighbor, graph, seen, ans);
                    }
                }
            }
            *ans
        }
        
        
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..n {
            graph.insert(i, Vec::new());
        }
        
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            graph.entry(x).or_insert(Vec::new()).push(y);
            graph.entry(y).or_insert(Vec::new()).push(x);
        }
        
        let mut seen = HashSet::new();
        for n in restricted {
            seen.insert(n);
        }
        
        let mut ans = 0;
        seen.insert(0);
        dfs(0, &graph, &mut seen, &mut ans)
        
}

// time and space complexities of O(V + E);
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

use std::collections::{HashMap, HashSet};

fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut seen: HashSet<i32> = HashSet::new();
        
        fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut HashSet<i32>) {
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if !seen.contains(&neighbor) {
                        seen.insert(*neighbor);
                        dfs(*neighbor, graph, seen);
                    }
                }
            }
            
        }
        
        for i in 0..n {
            graph.insert(i, Vec::new());
        }
        
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            graph.entry(x).or_insert(Vec::new()).push(y);
            graph.entry(y).or_insert(Vec::new()).push(x);
        }

        let mut ans = 0;
        for i in 0..n {
            if!seen.contains(&i) {
                ans += 1;
                seen.insert(i);
                dfs(i, &graph, &mut seen);
            }
        }
        
        ans
}

// time and space complexities O(V + E);
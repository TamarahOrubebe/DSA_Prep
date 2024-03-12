/*
Find if Path Exists in Graph

Solution
There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1 (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.

You want to determine if there is a valid path that exists from vertex source to vertex destination.

Given edges and the integers n, source, and destination, return true if there is a valid path from source to destination, or false otherwise.

 

Example 1:


Input: n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
Output: true
Explanation: There are two paths from vertex 0 to vertex 2:
- 0 → 1 → 2
- 0 → 2
Example 2:


Input: n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
Output: false
Explanation: There is no path from vertex 0 to vertex 5.
 

Constraints:

1 <= n <= 2 * 105
0 <= edges.length <= 2 * 105
edges[i].length == 2
0 <= ui, vi <= n - 1
ui != vi
0 <= source, destination <= n - 1
There are no duplicate edges.
There are no self edges.

*/

use std::collections::{HashMap, HashSet};

fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut seen = HashSet::new();
        
        fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut HashSet<i32>, destination: i32) -> bool {
            
            if node == destination {
                return true;
            }
            
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                     if !seen.contains(neighbor) {
                        seen.insert(*neighbor);
                        if dfs(*neighbor, graph, seen, destination){
                            return true;
                         } 
                    }
                }
               
            }
            false
        }
        
        
        for i in 0..n {
            graph.insert(i, Vec::new());
        }
        
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            graph.entry(x).or_insert(Vec::new()).push(y);
            graph.entry(y).or_insert(Vec::new()).push(x)
           
        }
    
        seen.insert(source);
        
        dfs(source, &graph, &mut seen, destination)
}

// time and space complexities of O(v + e);
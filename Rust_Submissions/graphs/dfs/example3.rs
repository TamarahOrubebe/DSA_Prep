/*
Example 3: 1466. Reorder Routes to Make All Paths Lead to the City Zero

There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between 
two different cities. Roads are represented by connections where connections[i] = [x, y] represents a road from 
city x to city y. The edges are directed. You need to swap the direction of some edges so that every city can 
reach city 0. Return the minimum number of swaps needed.


Because there is only one way to travel between any two cities (the problem explicitly states this), the only way
that every city can reach 0 is if every edge's direction is toward 0. That means we need to identify all edges 
that are pointing away from 0.

Let's say we start a DFS from 0. Because we use seen to prevent revisiting nodes, we will always be moving away 
from 0 (since we started there).

If we are at node A and we move to node B, we know the edge A -> B must be pointing away from 0. We can put all 
the edges from the input into a set, and then when we perform the traversal, we check if each 
edge node -> neighbor is in the set. If node -> neighbor is in the set, it means that it was in the original 
input and since it is pointing away from 0, we must increment our answer as it is an edge that needs to be 
flipped. The reason we put the original edges in a set is so that this check can be O(1).

The only problem is that because the edges are directed, starting a DFS from 0 will likely not visit every node.
 To remedy this, when we build our graph from the input (like we did in the first example), we can treat the 
 edges as undirected. This is solely to enable us to perform the DFS starting from 0.

Then we start a DFS from 0 and every node -> neighbor edge we encounter that is in the original input must be 
flipped.



*/

use std::collections::{HashSet, HashMap};



fn min_reorder(n: usize, connections: &Vec<[usize; 2]>) -> i32 {
    

    fn dfs(node: usize, graph: &HashMap<usize, Vec<usize>>, roads: &HashSet<(usize, usize)>, seen: &mut Vec<bool>) -> i32 {
        let mut ans = 0;
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !seen[neighbor] {
                    if roads.contains(&(node, neighbor)) {
                        ans += 1;
                    }
                    seen[neighbor] = true;
                    ans += dfs(neighbor, graph, roads, seen);
                }
            }
        }
        ans
    }

    let mut roads = HashSet::new();
    let mut graph = HashMap::new();
    let mut seen = vec![false; n];
    
    for i in 0..n {
        graph.insert(i, vec![]);
    }
    
     for &[x, y] in connections {
        graph.entry(x).and_modify(|e| e.push(y));
        graph.entry(y).and_modify(|e| e.push(x));
        roads.insert((x, y));
    }
    
    seen[0] = true;
    dfs(0, &graph, &roads, &mut seen)
}

fn main() {
    // Example usage
    let n = 6;
    let connections = vec![[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]];
    let result = min_reorder(n, &connections);
    println!("Minimum reorder count: {}", result);
}

//The time and space complexity of this algorithm is O(n) because we only visit each node once, do constant
// work, and are told the number of edges is n - 1 = O(n).roads, graph, and seen all take up at most 
// O(n) space.
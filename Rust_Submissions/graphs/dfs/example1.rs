/*
Example 1: 547. Number of Provinces

There are n cities. A province is a group of directly or indirectly connected cities and no other cities outside 
of the group. You are given an n x n matrix isConnected where isConnected[i][j] = isConnected[j][i] = 1 if the 
ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise. Return the total number 
of provinces.

Depending on the language you're using, if your keys are well defined (like the integers between 0 to n - 1), 
it might be better to use a boolean array instead of a set to implement seen.


We are told that there are n cities, with some cities being connected.

We can treat each of the cities as a node, with each city labeled between 0 to n - 1.

The connections (edges) are given to us in the input. Before we start any traversal, we can first build a graph 
so that we can easily access any given node's neighbors. If this is your first time solving graph problems, 
don't worry. This is a standard first step and the code used to do it is very similar across all problems. 
We iterate over all pairs of cities (i, j), and if isConnected[i][j] = 1, we add an undirected edge between i 
and j.

Next, how do we count the provinces? In a province, you can start at any city and find a path to reach any 
other city. This means that they form a connected component.

With an undirected graph, a traversal on a given node will visit every node in the connected component node 
belongs to. This is a property you can memorize, but it also makes sense if you think about it. Remember that 
in a binary tree, if you did a traversal starting from the root, you would visit every node in the tree.

Knowing that a DFS will visit every node in a connected component, we can use a data structure seen to tell us 
which cities we have already visited. This is another standard idea in all graph problems to avoid visiting a 
node multiple times. When we are performing the traversal and are at a given node, we iterate over the neighbors.
 For each neighbor, we first check if neighbor has been visited. If it has, we ignore it. If it hasn't, we mark 
 it as visited in seen and then recursively call dfs(neighbor) (just like we did with binary trees).

We iterate over the cities, and if we find a city i is not visited yet, we can perform a DFS starting from i. 
As we know, this traversal will visit every node in the connected component that i belongs to (aka, the province 
that i belongs to). After the traversal, seen will be updated with the entire province. We can increment the 
answer, and we don't need to worry about the province we just visited anymore because seen will prevent us from 
revisiting it.

*/

use std::collections::HashMap;

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    fn dfs(node: usize, graph: &HashMap<usize, Vec<usize>>, seen: &mut Vec<bool>) {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                // the next 2 lines are needed to prevent cycles
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    dfs(neighbor, graph, seen);
                }
            }
        }
    }

    // Build the graph
    let n = is_connected.len();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        graph.insert(i, Vec::new());
    }

    for i in 0..n {
        for j in i + 1..n {
            if is_connected[i][j] == 1 {
                graph.get_mut(&i).unwrap().push(j);
                graph.get_mut(&j).unwrap().push(i);
            }
        }
    }

    let mut seen = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if !seen[i] {
            ans += 1;
            seen[i] = true;
            dfs(i, &graph, &mut seen);
        }
    }

    ans
}

fn main() {
    // Example usage
    let is_connected = vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ];
    let result = find_circle_num(is_connected);
    println!("Number of connected circles: {}", result);
}

/* 
The time complexity of DFS on a graph is slightly different than when it is on a binary tree. With binary tree 
questions, we argued that each node is visited at most once, and each visit cost O(1) With graphs, we also 
only visit each node at most once, but the work is not necessarily O(1), because there is a for loop that 
iterates over the node's neighbors.

The reason visits were O(1) in a binary tree is because a node could have at most 2 children/neighbors, so 
we didn't need a loop. We just referenced node.left and node.right. With a graph, a node could have any amount 
of neighbors, so we need a non-constant loop.

As such, the time complexity for DFS on graphs is usually O(n + e), where nn is the number of nodes and 
ee is the number of edges. In the worst-case scenario where every node is connected with every other node, 
e = n^2
 .

Each node is visited only once
We iterate over a node's edges only when we are visiting that node
Because we can only visit a node once, a node's edges are only iterated over once
Therefore, all edges are iterated over only once, which costs O(e)O(e)
This is similar to the argument we made in the sliding window article that justified an O(n) time complexity 
despite the nested while loop. The nested while loop could only iterate nn times across the entire algorithm. 
Here, the for loop inside the function iterates ee times total across the entire algorithm.

Technically in this problem, the time complexity is O(n^2) because the input is given as an adjacency matrix, so 
we always need O(n^2) to build the hash map. The e is dominated by n^2 (because O(e < n^2)), so it can be ignored.

What about the space complexity? When we build graph, we are storing all the edges in arrays. We also need some 
space for the recursion call stack (O(n) in the worst case) as well as for seen. Therefore the space 
complexity is O(n + e).

The space complexity isn't O(n^2) because e is not necessarily dominated. In the worst case scenario, e = n^2 but 
e is still independent of n. In the time complexity, we always iterated over the entire matrix to build 
the graph, but in terms of space complexity, the hash map only grows if the edges actually exist.
*/
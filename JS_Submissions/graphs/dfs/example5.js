/*
Example 5: 1557. Minimum Number of Vertices to Reach All Nodes

Given a directed acyclic graph, with n vertices numbered from 0 to n-1, and an array edges where 
edges[i] = [x, y] represents a directed edge from node x to node y. Find the smallest set of vertices from which
all nodes in the graph are reachable.

The problem wants the smallest set of nodes from which all other nodes can be reached. This can be rephrased as 
the smallest set of nodes that cannot be reached from other nodes, because if a node can be reached from 
another node, then we would rather just include the "parent" rather than the "child" in our set.

A node cannot be reached from another node if it has an indegree of 0 (no edges are entering the node). 
Therefore, we can just find the indegree of all nodes and only include the ones with a zero indegree.

Note: if the graph had cycles, we would run into some edge cases. Imagine if the graph was just one cycle 
(a circle). Which node do we return? Technically, returning any of them would be correct. Our algorithm, 
however, would return nothing because none of the nodes would have an indegree of 0. Fortunately, the given 
graph is acyclic, so we don't have to worry about these cases.

*/

/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findSmallestSetOfVertices = function(n, edges) {
    let indegree = new Array(n).fill(0);
    for (const [x, y] of edges) {
        indegree[y]++;
    }
    
    let ans = [];
    for (let i = 0; i < n; i++) {
        if (indegree[i] == 0) {
            ans.push(i);
        }
    }
    
    return ans;
};

// This example doesn't require a DFS, but is a good exercise to better understand the mechanics of graphs. 

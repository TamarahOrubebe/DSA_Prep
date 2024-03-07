/*
Example 4: 841. Keys and Rooms

There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. Your goal is to visit 
all the rooms. When you visit a room, you may find a set of distinct keys in it. Each key has a number on it, 
denoting which room it unlocks, and you can take all of them with you to unlock the other rooms. Given an array 
rooms where rooms[i] is the set of keys that you can obtain if you visited room i, return true if you can visit 
all the rooms, or false otherwise.

Every room is locked except 0. This indicates that we can start at 0. When we visit a room, we find some keys 
that enable us to visit other rooms. This tells us we can model the problem as a graph. The rooms are nodes and 
the keys are edges.

The input of the graph is the most convenient one - an adjacency list. We don't need to build graph like we did 
in the previous examples because the input already serves that function - if we want to find the neighbors of a 
given node i, we can simply check rooms[i].

The problem is asking if we can visit all rooms. This is equivalent to "starting a DFS from 0, can you visit all
nodes?".

So we simply start a DFS from 0, and check if we visited all the nodes after the traversal finishes. Because we 
add a node to seen every time we visit it, we can simply compare the size of seen against n (which is the length 
of rooms since the input is an adjacency list).
*/

/**
 * @param {number[][]} rooms
 * @return {boolean}
 */
var canVisitAllRooms = function(rooms) {
    let dfs = node => {
        for (const neighbor of rooms[node]) {
            if (!seen.has(neighbor)) {
                seen.add(neighbor);
                dfs(neighbor);
            }
        }
    }
    
    let seen = new Set([0]);
    dfs(0);
    return seen.size == rooms.length;
};


/*
Adjacency lists are the most convenient input format when the nodes are numbered from 0 to n - 1 because we 
don't need to convert it to a hash map - it basically is already in that format. As such, the only extra space 
we use here is in seen and the recursion call stack, which both are O(n). The time complexity is O(n + e)
as we visit each node once and the for loops inside each visit will iterate up to ee times total across 
the entire algorithm.
*/
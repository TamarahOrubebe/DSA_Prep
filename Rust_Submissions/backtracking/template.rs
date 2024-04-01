/*
Implementation

Backtracking is almost always implemented with recursion - it really doesn't make sense to do it iteratively. In 
most backtracking problems, you will be building something, either directly (like modifying an array) or 
indirectly (using variables to represent some state). Here is some pseudocode for a general backtracking format:

// let curr represent the thing you are building
// it could be an array or a combination of variables

function backtrack(curr) {
    if (base case) {
        Increment or add to answer
        return
    }

    for (iterate over input) {
        Modify curr
        backtrack(curr)
        Undo whatever modification was done to curr
    }
}
Let's think back to the analogy of possibilities being represented by a tree.

Each call to the function backtrack represents a node in the tree. Each iteration in the for loop represents a 
child of the current node, and calling backtrack in that loop represents moving to a child.

The line where you undo the modifications is the "backtracking" step and is equivalent to moving back up the 
tree from a child to its parent.

At any given node, the path from the root to the node represents a candidate that is being built. The leaf nodes
are complete solutions and represent when the base case is reached. The root of this tree is an empty candidate 
and represents the scope that the original backtrack call is being made from.

*/

fn main () {
    println!("Backtracking template");
}
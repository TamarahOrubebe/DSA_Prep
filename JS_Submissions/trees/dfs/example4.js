/*
Example 4: 100. Same Tree

Given the roots of two binary trees p and q, check if they are the same tree. Two binary trees are the same tree 
if they are structurally identical and the nodes have the same values.

This problem really demonstrates the recursive nature of binary trees.

If p and q are the same tree, then the following is true:

p.val = q.val
p.left and q.left are the same tree
p.right and q.right are the same tree
The main idea is that if any two trees are the same, then their subtrees must also be the same. This gives us a 
recursive definition of the problem. Because the function we are trying to implement is supposed to tell us if 
two trees are the same, we can use the function itself to answer conditions 2 and 3.

The following condition can be used to check if p and q are the same tree:

p.val == q.val && isSameTree(p.left, q.left) && isSameTree(p.right, q.right)

Now, we need base cases so that the recursion eventually terminates. If p and q are both null, then we can 
return true, because they are technically both the same (empty) tree. If either p or q is null but not the other,
 we should return false, as they are clearly not the same tree.

A good way to think about base cases is to think about a tree with only one node. Let's say that p and q are 
both one-node trees with the same value. The first boolean check p.val == q.val passes, so now we check the 
subtrees. Because the nodes don't have children, then both calls to the left and right subtrees will trigger 
the base case and return true.

This is the beauty of recursion - if you're at the root, the left and right subtrees could have thousands of 
nodes. The process of actually going through the trees will have many cascading calls, but you don't need to 
worry about it - you know that simply making the call will give you the answer you need.


*/

var isSameTree = function(p, q) {
    if (p == null && q == null) {
        return true;
    }
    
    if (p == null || q == null) {
        return false;
    }
    
    if (p.val != q.val) {
        return false;
    }
    
    let left = isSameTree(p.left, q.left);
    let right = isSameTree(p.right, q.right);
    return left && right;
};

// time and space complexity O(n);
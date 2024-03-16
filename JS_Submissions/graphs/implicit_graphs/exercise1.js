/*
 Minimum Genetic Mutation

Solution
A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.

Suppose we need to investigate a mutation from a gene string startGene to a gene string endGene where one 
mutation is defined as one single character changed in the gene string.

For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a 
valid gene string.

Given the two gene strings startGene and endGene and the gene bank bank, return the minimum number of mutations 
needed to mutate from startGene to endGene. If there is no such a mutation, return -1.

Note that the starting point is assumed to be valid, so it might not be included in the bank.

 

Example 1:

Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
Output: 1
Example 2:

Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
Output: 2
 

Constraints:

0 <= bank.length <= 10
startGene.length == endGene.length == bank[i].length == 8
startGene, endGene, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
*/

/**
 * @param {string} startGene
 * @param {string} endGene
 * @param {string[]} bank
 * @return {number}
 */
var minMutation = function(startGene, endGene, bank) {
    
    let neighbors = node => {
        let ans = [];
        let chars = ["A", "C", "G", "T"];
        for (let i = 0; i < 8; i++) {
            let num = node[i];
            for (const char of chars) {
                ans.push(node.slice(0, i) + char + node.slice(i + 1));
            }
        }
        
        return ans;
    }
    
    
    let queue = [startGene];
    let isValid = new Set(bank);
    let seen = new Set();
    seen.add(startGene);
    
    let steps = 0;
    
    while (queue.length) {
        let currentLength = queue.length;
        let nextQueue = [];
        
        for (let i = 0; i < currentLength; i++) {
            const node = queue[i];
            if (node == endGene) {
                return steps;
            }
            
            for (const neighbor of neighbors(node)) {
                if(!isValid.has(neighbor)) continue;
                if (!seen.has(neighbor)) {
                    seen.add(neighbor);
                    nextQueue.push(neighbor);
                }
            }
        }
        
        steps++;
        queue = nextQueue;
    }
    
    return -1;
};
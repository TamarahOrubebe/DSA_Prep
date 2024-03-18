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

use std::collections::VecDeque;
use std::collections::HashSet;

fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        if bank.len() == 0 {
            return -1;
        }
        fn neighbors(node: &str) -> Vec<String> {
            let mut ans = Vec::new();
            let chars = vec!['A', 'C', 'G', 'T'];
            
            for i in 0..node.len() {
                for char in &chars {
                    ans.push(format!("{}{}{}", &node[..i], *char, &node[i + 1..] ));
                }
            }
            ans
        }
        
        
        
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut is_valid: HashSet<String> = HashSet::new();
        for i in bank {
            is_valid.insert(i);
        }
        let mut seen = HashSet::new();
        seen.insert(start_gene.clone());
        let mut steps = 0;
        queue.push_back(start_gene);
        while !queue.is_empty() {
            let curr_len = queue.len();
            
            for i in 0..curr_len {
                if let Some(node) = queue.pop_front() {
                    if node == end_gene {
                        return steps;
                    }
                    
                    for neighbor in neighbors(&node) {
                        if !is_valid.contains(&neighbor) {
                            continue;
                        }
                        
                        if !seen.contains(&neighbor) {
                            seen.insert(neighbor.clone());
                            queue.push_back(neighbor)
                        }
                           
                    }
                }
            }
            
            steps += 1;
        }
        -1
}




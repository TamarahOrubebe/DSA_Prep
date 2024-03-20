/*
Detonate the Maximum Bombs

Solution
You are given a list of bombs. The range of a bomb is defined as the area where its effect can be felt. This area
is in the shape of a circle with the center as the location of the bomb.

The bombs are represented by a 0-indexed 2D integer array bombs where bombs[i] = [xi, yi, ri]. xi and yi denote 
the X-coordinate and Y-coordinate of the location of the ith bomb, whereas ri denotes the radius of its range.

You may choose to detonate a single bomb. When a bomb is detonated, it will detonate all bombs that lie in its 
range. These bombs will further detonate the bombs that lie in their ranges.

Given the list of bombs, return the maximum number of bombs that can be detonated if you are allowed to detonate 
only one bomb.

 

Example 1:


Input: bombs = [[2,1,3],[6,1,4]]
Output: 2
Explanation:
The above figure shows the positions and ranges of the 2 bombs.
If we detonate the left bomb, the right bomb will not be affected.
But if we detonate the right bomb, both bombs will be detonated.
So the maximum bombs that can be detonated is max(1, 2) = 2.
Example 2:


Input: bombs = [[1,1,5],[10,10,5]]
Output: 1
Explanation:
Detonating either bomb will not detonate the other bomb, so the maximum number of bombs that can be detonated is 
1.
Example 3:


Input: bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
Output: 5
Explanation:
The best bomb to detonate is bomb 0 because:
- Bomb 0 detonates bombs 1 and 2. The red circle denotes the range of bomb 0.
- Bomb 2 detonates bomb 3. The blue circle denotes the range of bomb 2.
- Bomb 3 detonates bomb 4. The green circle denotes the range of bomb 3.
Thus all 5 bombs are detonated.
 

Constraints:

1 <= bombs.length <= 100
bombs[i].length == 3
1 <= xi, yi, ri <= 105
*/

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let len = bombs.len();
        
        for i in 0..len {
            graph.insert(i as i32, Vec::new());
        }
        
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }
                
                let (xi, yi, ri) = (bombs[i][0] as i64, bombs[i][1] as i64, bombs[i][2] as i64);
                let (xj, yj, rj) = (bombs[j][0] as i64, bombs[j][1] as i64, bombs[j][2] as i64);
                
                if ri*ri >= (xi - xj).pow(2)  + (yi - yj).pow(2) {
                   if let Some(neighbors) = graph.get_mut(&(i as i32)) {
                       neighbors.push(j as i32);
                    }
                }
            }
        }
        
        fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>) -> i32 {
           visited[node as usize] = true;
           let mut size = 1; 
           if let Some(neighbors) = graph.get(&node) {
               for neighbor in neighbors {
                   if !visited[*neighbor as usize] {
                      size += dfs(*neighbor,graph, visited);
                   }
               }
            }
            size
        }
        
        let mut ans = 0;
        let mut visited  = vec![false; bombs.len()];
        
        for i in 0..len {
            ans = ans.max(dfs(i as i32, &graph, &mut visited.clone()));
        }
        ans
    }
}
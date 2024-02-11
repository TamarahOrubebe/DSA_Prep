/* 
Find Players With Zero or One Losses

Solution
You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.

Return a list answer of size 2 where:

answer[0] is a list of all players that have not lost any matches.
answer[1] is a list of all players that have lost exactly one match.
The values in the two lists should be returned in increasing order.

Note:

You should only consider the players that have played at least one match.
The testcases will be generated such that no two matches will have the same outcome.
 

Example 1:

Input: matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
Output: [[1,2,10],[4,5,7,8]]
Explanation:
Players 1, 2, and 10 have not lost any matches.
Players 4, 5, 7, and 8 each have lost one match.
Players 3, 6, and 9 each have lost two matches.
Thus, answer[0] = [1,2,10] and answer[1] = [4,5,7,8].
Example 2:

Input: matches = [[2,3],[1,3],[5,4],[6,4]]
Output: [[1,2,5,6],[]]
Explanation:
Players 1, 2, 5, and 6 have not lost any matches.
Players 3 and 4 each have lost two matches.
Thus, answer[0] = [1,2,5,6] and answer[1] = [].
 

Constraints:

1 <= matches.length <= 105
matches[i].length == 2
1 <= winneri, loseri <= 105
winneri != loseri
All matches[i] are unique.

*/

fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let mut players = HashSet::new();
    let mut losers = HashMap::new();
    let mut ans = vec![vec![], vec![]];
        
        
    for m in matches {
        let w = m[0];
        let l = m[1];
        players.insert(w);
        players.insert(l);
        *losers.entry(l).or_insert(0) += 1;
    }
        
       
    for player in players {
        if !losers.contains_key(&player) {
            ans[0].push(player);
        }
            
        if losers.get(&player) == Some(&1) {
            ans[1].push(player);
        }
    }
        

    for a in ans.iter_mut() {
        a.sort();
    }
        
    ans
}

// time complexity of O(nlogn) space complexity of O(n)
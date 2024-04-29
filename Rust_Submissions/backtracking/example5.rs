/*
Example 3: 79. Word Search

Given an m x n grid of characters board and a string word, return true if word exists in the grid. The word can 
be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically 
neighboring. The same letter cell may not be used more than once.
*/

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn backtrack(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        row: i32,
        col: i32,
        index: usize,
        seen: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        
         fn valid (row: i32, col: i32, m: i32, n: i32) -> bool {
            0 <= row && row < m && 0 <= col && col < n
        }
        
        for &(dx, dy) in directions.iter() {
            let next_row = row + dy;
            let next_col = col + dx;
            if valid(next_row, next_col, board.len() as i32, board[0].len() as i32)
                && !seen[next_row as usize][next_col as usize]
            {
                if board[next_row as usize][next_col as usize] == word[index] {
                    seen[next_row as usize][next_col as usize] = true;
                    if backtrack(
                        board,
                        word,
                        next_row,
                        next_col,
                        index + 1,
                        seen,
                        directions,
                    ) {
                        return true;
                    }
                    seen[next_row as usize][next_col as usize] = false;
                }
            }
        }

        false
    }

    let m = board.len() as i32;
    let n = board[0].len() as i32;

    for row in 0..m {
        for col in 0..n {
            let mut seen = vec![vec![false; n as usize]; m as usize];
            seen[row as usize][col as usize] = true;
            if board[row as usize][col as usize] == word.chars().nth(0).unwrap()
                && backtrack(
                    &board,
                    &word.chars().collect(),
                    row,
                    col,
                    1,
                    &mut seen,
                    &directions,
                )
            {
                return true;
            }
        }
    }

    false
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = String::from("ABCCED");
    println!("{}", exist(board, word));
}
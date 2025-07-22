struct Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
            let rows = board.len();
            let cols = board[0].len();
            if board[i][j] != 'O' {
                return;
            }
            board[i][j] = 'T';
            if i > 0 {
                dfs(board, i - 1, j);
            }
            if i < rows - 1 {
                dfs(board, i + 1, j);
            }
            if j > 0 {
                dfs(board, i, j - 1);
            }
            if j < cols - 1 {
                dfs(board, i, j + 1);
            }
        }
        if board.is_empty() {
            return;
        }
        let rows = board.len();
        let cols = board[0].len();
        // 只有边界这一圈需要触发dfs
        for i in 0..rows {
            dfs(board, i, 0);
            dfs(board, i, cols - 1);
        }
        for j in 0..cols {
            dfs(board, 0, j);
            dfs(board, rows - 1, j);
        }
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        fn is_valid_box(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
            let mut seen = [false; 9];
            for i in 0..3 {
                for j in 0..3 {
                    let c = board[row + i][col + j];
                    if c != '.' {
                        let index = (c as u8 - b'1') as usize;
                        if seen[index] {
                            return false;
                        }
                        seen[index] = true;
                    }
                }
            }
            true
        }

        fn is_valid_row(row: &Vec<char>) -> bool {
            let mut seen = [false; 9];
            for &c in row {
                if c != '.' {
                    let index = (c as u8 - b'1') as usize;
                    if seen[index] {
                        return false;
                    }
                    seen[index] = true;
                }
            }
            true
        }

        fn is_valid_col(board: &Vec<Vec<char>>, col: usize) -> bool {
            let mut seen = [false; 9];
            for row in board {
                let c = row[col];
                if c != '.' {
                    let index = (c as u8 - b'1') as usize;
                    if seen[index] {
                        return false;
                    }
                    seen[index] = true;
                }
            }
            true
        }
        for row in &board {
            if !is_valid_row(row) {
                return false;
            }
        }
        for col in 0..9 {
            if !is_valid_col(&board, col) {
                return false;
            }
        }

        for row in (0..9).step_by(3) {
            for col in (0..9).step_by(3) {
                if !is_valid_box(&board, row, col) {
                    return false;
                }
            }
        }

        true
    }
}

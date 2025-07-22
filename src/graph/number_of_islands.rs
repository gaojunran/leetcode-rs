struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            let rows = grid.len();
            let cols = grid[0].len();

            if grid[i][j] == '0' {
                return;
            }

            // 标记为已访问
            grid[i][j] = '0';

            // 上下左右递归
            if i > 0 {
                dfs(grid, i - 1, j);
            }
            if j > 0 {
                dfs(grid, i, j - 1);
            }
            if i + 1 < rows {
                dfs(grid, i + 1, j);
            }
            if j + 1 < cols {
                dfs(grid, i, j + 1);
            }
        }

        let mut count = 0;
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    count += 1;
                    dfs(&mut grid, i, j);
                }
            }
        }

        count
    }
}

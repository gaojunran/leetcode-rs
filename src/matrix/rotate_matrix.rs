struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut new_matrix = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                new_matrix[j][n - 1 - i] = matrix[i][j];
            }
        }

        // 将 new_matrix 拷贝回 matrix
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = new_matrix[i][j];
            }
        }
    }
}

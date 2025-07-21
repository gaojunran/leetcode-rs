struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();

        // 先记录要清零的行和列
        let mut zero_rows = vec![false; rows];
        let mut zero_cols = vec![false; cols];

        for (i, row) in matrix.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 0 {
                    zero_rows[i] = true;
                    zero_cols[j] = true;
                }
            }
        }

        // 清零行
        matrix.iter_mut().enumerate().for_each(|(i, row)| {
            if zero_rows[i] {
                row.iter_mut().for_each(|x| *x = 0);
            }
        });

        // 清零列
        (0..cols).filter(|&j| zero_cols[j]).for_each(|j| {
            matrix.iter_mut().for_each(|row| row[j] = 0);
        });
    }
}

// impl Solution {
//     pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
//         let rows = matrix.len();
//         let cols = matrix[0].len();

//         let mut zero_rows = vec![false; rows];
//         let mut zero_cols = vec![false; cols];

//         // 第一步：标记要清零的行和列
//         for (i, row) in matrix.iter().enumerate() {
//             for (j, &val) in row.iter().enumerate() {
//                 if val == 0 {
//                     zero_rows[i] = true;
//                     zero_cols[j] = true;
//                 }
//             }
//         }

//         // 第二步：一口气清零
//         for i in 0..rows {
//             for j in 0..cols {
//                 if zero_rows[i] || zero_cols[j] {
//                     matrix[i][j] = 0;
//                 }
//             }
//         }
//     }
// }

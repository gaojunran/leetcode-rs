struct Solution;
// impl Solution {
//     pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
//         if matrix.len() == 1 {
//             return matrix[0].clone();
//         }
//         if matrix[0].len() == 1 {
//             return matrix.into_iter().flatten().collect();
//         }
//         let (mut left, right, mut top, bottom) = (0, matrix[0].len() - 1, 0, matrix.len() - 1);
//         let mut step = matrix[0].len() * matrix.len();
//         let mut ans = Vec::with_capacity(step);
//         let mut flag = 0; //执行方向0-3
//         while step > 0 {
//             ans.push(matrix[top][left]);
//             match flag % 4 {
//                 0 => {
//                     // 向右
//                     left += 1;
//                     if left >= right - flag / 4 {
//                         flag += 1;
//                     }
//                 }
//                 1 => {
//                     // 向下
//                     top += 1;
//                     if top >= bottom - flag / 4 {
//                         flag += 1;
//                     }
//                 }
//                 2 => {
//                     // 向左
//                     left -= 1;
//                     if left <= flag / 4 {
//                         flag += 1;
//                     }
//                 }
//                 3 => {
//                     // 向上
//                     top -= 1;
//                     if top <= flag / 4 + 1 {
//                         flag += 1;
//                     }
//                 }
//                 _ => {}
//             };
//             step -= 1;
//         }
//         ans
//     }
// }
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右下左上

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = Vec::with_capacity(m * n);
        let mut i = 0;
        let mut j = 0;
        let mut di = 0;
        for _ in 0..m * n {
            // 一共走 mn 步
            ans.push(matrix[i as usize][j as usize]);
            matrix[i as usize][j as usize] = i32::MAX; // 标记，表示已经访问过（已经加入答案）
            let x = i + DIRS[di].0;
            let y = j + DIRS[di].1; // 下一步的位置
            // 如果 (x, y) 出界或者已经访问过
            if x < 0
                || x >= m as i32
                || y < 0
                || y >= n as i32
                || matrix[x as usize][y as usize] == i32::MAX
            {
                di = (di + 1) % 4; // 右转 90°
            }
            i += DIRS[di].0;
            j += DIRS[di].1; // 走一步
        }
        ans
    }
}

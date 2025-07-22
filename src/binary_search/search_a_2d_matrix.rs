struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n;
        while left < right {
            // 左闭右开
            let mid = left + (right - left) / 2;
            let x = matrix[mid / n][mid % n];
            if x == target {
                return true;
            }
            if x < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        false
    }
}

use std::cmp::{max, min};

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut maxv = 0;
        while i < j {
            let v = (j - i) as i32 * min(height[i], height[j]);
            maxv = max(maxv, v);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        maxv
    }
}

struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows;
        if n == 1 {
            return s;
        }
        let mut mat = vec![Vec::new(); n as usize];
        // for _ in 0..n {
        //     mat.push(Vec::new());
        // }
        let mut row = 0;
        // let mut col = 0;
        let mut is_slope = false;
        for &ch in s.as_bytes() {
            mat[row].push(ch);
            if row == 0 {
                is_slope = false;
            }
            if row == (n - 1) as usize {
                is_slope = true;
            }
            if is_slope {
                row -= 1;
                // col += 1;
            } else {
                row += 1;
            }
        }
        String::from_utf8(mat.iter().flatten().copied().collect()).unwrap()
    }
}

struct Solution;
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let n = citations.len();

        for i in 0..n {
            let h = (n - i) as i32;
            if citations[i] >= h {
                return h;
            }
        }

        // for h in (0..=*citations.last().unwrap()).rev() {
        //     let count = citations.iter().filter(|&&c| c >= h).count();
        //     if count >= h as usize {
        //         return h;
        //     }
        // }

        0
    }
}

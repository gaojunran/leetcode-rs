struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // let mut meet = 0;
        let mut i = 0;
        // let mut j = 0;
        for cht in t.chars() {
            match s.chars().nth(i) {
                Some(chs) if cht == chs => i += 1,
                None => return true,
                _ => {}
            }
        }
        i == s.len()
    }
}

// impl Solution {
//     pub fn is_subsequence(s: String, t: String) -> bool {
//         let mut s_iter = s.chars();
//         t.chars().any(|c| s_iter.next_if(|&sc| sc == c).is_some());
//         s_iter.next().is_none()
//     }
// }

// impl Solution {
//     pub fn is_subsequence(s: String, t: String) -> bool {
//         let mut s_iter = s.chars();
//         let mut current = s_iter.next();

//         for ch in t.chars() {
//             if let Some(sc) = current {
//                 if ch == sc {
//                     current = s_iter.next();
//                 }
//             } else {
//                 break;
//             }
//         }

//         current.is_none()
//     }
// }

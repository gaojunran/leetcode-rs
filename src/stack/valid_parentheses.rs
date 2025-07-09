use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let left = [b'[', b'(', b'{'];
        let right = [b']', b')', b'}'];
        let map: HashMap<_, _> = HashMap::from_iter(left.iter().zip(right.iter()));
        for &ch in s.as_bytes() {
            if left.contains(&ch) {
                stack.push(ch);
            } else if right.contains(&ch) {
                let l = match stack.pop() {
                    Some(v) => v,
                    None => return false,
                };
                if let Some(&&r) = map.get(&l) {
                    if r != ch {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

// impl Solution {
//     pub fn is_valid(s: String) -> bool {
//         let mut stack = Vec::new();

//         for ch in s.bytes() {
//             match ch {
//                 b'(' | b'[' | b'{' => stack.push(ch),
//                 b')' => {
//                     if stack.pop() != Some(b'(') {
//                         return false;
//                     }
//                 }
//                 b']' => {
//                     if stack.pop() != Some(b'[') {
//                         return false;
//                     }
//                 }
//                 b'}' => {
//                     if stack.pop() != Some(b'{') {
//                         return false;
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         stack.is_empty()
//     }
// }

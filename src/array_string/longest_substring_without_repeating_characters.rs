struct Solution;
use std::collections::HashSet;

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let chars: Vec<char> = s.chars().collect();
//         let mut set = HashSet::new();
//         let (mut left, mut right, mut max_len) = (0, 0, 0);

//         while right < chars.len() {
//             if !set.contains(&chars[right]) {
//                 set.insert(chars[right]);
//                 right += 1;
//                 max_len = max_len.max(right - left);
//             } else {
//                 set.remove(&chars[left]);
//                 left += 1;
//             }
//         }

//         max_len as i32
//     }
// }

// use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..chars.len() {
            // 如果集合里已存在当前字符，则收缩左边界
            while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }
            // 把当前字符加入窗口
            set.insert(chars[right]);
            // 更新最大长度
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

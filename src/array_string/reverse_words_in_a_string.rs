struct Solution;
// impl Solution {
//     pub fn reverse_words(s: String) -> String {
//         let mut result = String::new();
//         let mut curr = String::new();
//         for (i, &c) in s.as_bytes().iter().enumerate().rev() {
//             let ch = c as char;
//             if (ch == ' ') || i == 0 {
//                 if i == 0 {
//                     // 处理最后一个单词，可能存在多余的空格
//                     curr.push(ch);
//                     curr = curr.trim_end().to_string();
//                 }
//                 if curr.is_empty() {
//                     // 应该被跳过的空格
//                     continue;
//                 }
//                 if !result.is_empty() {
//                     // 第一个单词前面不应该加空格
//                     result.push(' ');
//                 }
//                 result.push_str(
//                     &String::from_utf8(curr.as_bytes().iter().rev().cloned().collect()).unwrap(),
//                 );
//                 curr.truncate(0);
//             } else {
//                 curr.push(ch);
//             }
//         }
//         result.trim().to_string()
//     }
// }

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let bytes = s.as_bytes();
        let mut result = String::new();
        let mut word = Vec::new();

        let mut i = bytes.len();
        while i > 0 {
            i -= 1;
            let c = bytes[i];

            if c == b' ' {
                if !word.is_empty() {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    word.reverse();
                    result.push_str(std::str::from_utf8(&word).unwrap());
                    word.clear();
                }
            } else {
                word.push(c);
            }
        }

        // 处理最前面的单词（如果存在）
        if !word.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            word.reverse();
            result.push_str(std::str::from_utf8(&word).unwrap());
        }

        result
    }
}

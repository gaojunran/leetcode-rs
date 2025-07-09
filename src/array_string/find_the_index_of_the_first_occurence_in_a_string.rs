use std::ops::Index;

struct Solution;
// impl Solution {
//     pub fn str_str(haystack: String, needle: String) -> i32 {
//         let mut curr = -1;
//         let mut idx = 0; // needle's
//         if needle.len() > haystack.len() {
//             return curr;
//         }
//         for (i, &ch) in haystack.as_bytes().iter().enumerate() {
//             if needle.as_bytes()[idx] == ch {
//                 if idx == 0 {
//                     curr = i as i32;
//                 }
//                 if idx == needle.len() - 1 {
//                     break;
//                 }
//                 idx += 1;
//             } else {
//                 curr = -1;
//                 idx = 0;
//             }
//         }
//         curr
//     }
// }

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n = haystack.len();
        let m = needle.len();
        if m == 0 {
            return 0;
        }
        if m > n {
            return -1;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        // 构造 next 数组
        let mut next = vec![0; m];
        let mut j = 0; // 前缀指针
        for i in 1..m {
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            next[i] = j;
        }

        let mut idx = 0; // needle 的索引
        for (i, &ch) in haystack.iter().enumerate() {
            while idx > 0 && needle[idx] != ch {
                idx = next[idx - 1]; // 利用 next 回退 idx
            }

            if needle[idx] == ch {
                idx += 1;
                if idx == m {
                    return (i + 1 - m) as i32;
                }
            }
        }

        -1
    }
}

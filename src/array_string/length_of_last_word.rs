struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let n = s.len();
        let mut j = n - 1;
        while s.chars().nth(j).unwrap() == ' ' {
            j -= 1;
        }
        dbg!(j);
        let mut i = j;
        while s.chars().nth(i).unwrap_or(' ') != ' ' {
            i -= 1;
        }
        (j - i) as i32
    }
}

// impl Solution {
//     pub fn length_of_last_word(s: String) -> i32 {
//         let bytes = s.as_bytes();
//         let mut j = bytes.len();

//         // 从后往前跳过空格
//         while j > 0 && bytes[j - 1] == b' ' {
//             j -= 1;
//         }

//         // 统计最后一个单词的长度
//         let mut i = j;
//         while i > 0 && bytes[i - 1] != b' ' {
//             i -= 1;
//         }

//         (j - i) as i32
//     }
// }

// test: "a"
mod test {
    use super::Solution;

    #[test]
    fn test222() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
    }
}

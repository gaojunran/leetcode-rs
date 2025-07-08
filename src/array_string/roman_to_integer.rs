struct Solution;
// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         // let roman_map = std::collections::HashMap::from([
//         //     ('Q', 0),
//         //     ('I', 1),
//         //     ('V', 5),
//         //     ('X', 10),
//         //     ('L', 50),
//         //     ('C', 100),
//         //     ('D', 500),
//         //     ('M', 1000),
//         // ]);
//         // let mut cum = 0;
//         // for (idx, c) in s.chars().enumerate() {
//         //     let mut curr = *roman_map.get(&c).unwrap();
//         //     if *roman_map
//         //         .get(&s.chars().nth(idx + 1).unwrap_or('Q'))
//         //         .unwrap_or(&0)
//         //         > curr
//         //     {
//         //         curr *= -1;
//         //     }
//         //     cum += curr;
//         // }
//         // cum
//     }
// }

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0, // 非法字符默认为 0
            }
        }

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        for i in 0..chars.len() {
            let curr = value(chars[i]);
            let next = if i + 1 < chars.len() {
                value(chars[i + 1])
            } else {
                0
            };

            if curr < next {
                total -= curr;
            } else {
                total += curr;
            }
        }

        total
    }
}

// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         // 直接使用数组模拟字符值（ASCII 范围足够小）
//         let mut val = [0; 256];
//         val[b'I' as usize] = 1;
//         val[b'V' as usize] = 5;
//         val[b'X' as usize] = 10;
//         val[b'L' as usize] = 50;
//         val[b'C' as usize] = 100;
//         val[b'D' as usize] = 500;
//         val[b'M' as usize] = 1000;

//         let bytes = s.as_bytes();
//         let mut total = 0;
//         let mut i = 0;

//         while i < bytes.len() {
//             let curr = val[bytes[i] as usize];
//             let next = if i + 1 < bytes.len() {
//                 val[bytes[i + 1] as usize]
//             } else {
//                 0
//             };

//             if curr < next {
//                 total += next - curr;
//                 i += 2;
//             } else {
//                 total += curr;
//                 i += 1;
//             }
//         }

//         total
//     }
// }

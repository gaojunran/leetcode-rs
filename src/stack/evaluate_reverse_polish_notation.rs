struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for i in tokens {
            let i = i.as_str();
            match i {
                "+" | "-" | "*" | "/" => {
                    // 注意：先出栈的是右操作数
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let result = match i {
                        "+" => b + a,
                        "-" => b - a,
                        "*" => b * a,
                        "/" => b / a,
                        _ => 0,
                    };
                    stack.push(result);
                }
                _ => {
                    let result = i.parse::<i32>().unwrap();
                    stack.push(result);
                }
            }
        }
        stack.pop().unwrap()
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn eval_rpn(tokens: Vec<String>) -> i32 {
//         let ops: HashMap<&str, fn(i32, i32) -> i32> = {
//             let mut m = HashMap::new();
//             m.insert("+", |b, a| b + a);
//             m.insert("-", |b, a| b - a);
//             m.insert("*", |b, a| b * a);
//             m.insert("/", |b, a| b / a);
//             m
//         };

//         let mut stack = Vec::new();
//         for token in tokens {
//             if let Some(&op) = ops.get(token.as_str()) {
//                 let a = stack.pop().unwrap();
//                 let b = stack.pop().unwrap();
//                 stack.push(op(b, a));
//             } else {
//                 stack.push(token.parse::<i32>().unwrap());
//             }
//         }
//         stack.pop().unwrap()
//     }
// }

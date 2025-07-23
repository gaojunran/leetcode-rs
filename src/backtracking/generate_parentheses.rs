struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut path = String::new();

        fn backtrack(left: i32, right: i32, path: &mut String, result: &mut Vec<String>) {
            // left 和 right 分别记录还可以使用多少个左括号和右括号
            if left == 0 && right == 0 {
                result.push(path.clone());
                return;
            }

            // 放左括号
            if left > 0 {
                path.push('(');
                backtrack(left - 1, right, path, result);
                path.pop(); // 回溯
            }

            // 放右括号，必须保证右括号数量不超过左括号
            if right > left {
                path.push(')');
                backtrack(left, right - 1, path, result);
                path.pop(); // 回溯
            }
        }

        backtrack(n, n, &mut path, &mut result);
        result
    }
}

struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let parts = path.split('/'); // 按 '/' 拆分
        let mut ans = Vec::new();

        for part in parts {
            match part {
                "" | "." => {
                    // 空字符串和 "." 都忽略
                }
                ".." => {
                    ans.pop(); // 返回上一级，弹出栈顶元素（如果有的话）
                }
                _ => {
                    ans.push(part);
                }
            }
        }

        // 拼接结果，前面加 '/'
        format!("/{}", ans.join("/"))
    }
}

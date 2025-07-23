struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        fn dfs(i: usize, path: &mut [u8], digits: &[u8], mapping: &[&[u8]], ans: &mut Vec<String>) {
            if i == digits.len() {
                ans.push(String::from_utf8(path.to_vec()).unwrap());
                return;
            }
            for &c in mapping[(digits[i] - b'0') as usize] {
                path[i] = c;
                dfs(i + 1, path, digits, mapping, ans);
            }
        }

        let mapping: Vec<&[u8]> = vec![
            b"", b"", b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
        ];
        let digits = digits.as_bytes();
        let mut path = vec![0; digits.len()];
        let mut ans = vec![];
        dfs(0, &mut path, digits, &mapping, &mut ans);
        ans
    }
}

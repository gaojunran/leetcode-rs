struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let normalized: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut i = 0;
        let mut j = normalized.len().saturating_sub(1);

        while i < j {
            if normalized[i] != normalized[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

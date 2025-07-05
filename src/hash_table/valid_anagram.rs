struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = std::collections::HashMap::new();
        for chs in s.chars() {
            *map.entry(chs).or_insert(0) += 1;
        }
        for cht in t.chars() {
            // if let Some(v) = map.get_mut(&cht) {
            //     *v -= 1;
            //     if *v < 0 {
            //         return false;
            //     }
            // } else {
            //     return false;
            // }
            match map.get_mut(&cht) {
                Some(v) if *v > 0 => *v -= 1,
                _ => return false,
            }
        }
        true
    }
}

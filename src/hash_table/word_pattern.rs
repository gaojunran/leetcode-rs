struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split(" ").count() {
            return false;
        };

        let mut map = std::collections::HashMap::new();
        let mut remap = std::collections::HashMap::new();

        for (chp, chs) in pattern.chars().zip(s.split(" ")) {
            match (map.get(&chp), remap.get(&chs)) {
                (Some(&a), _) if a != chs => return false,
                (_, Some(&b)) if b != chp => return false,
                (None, None) => {
                    map.insert(chp, chs);
                    remap.insert(chs, chp);
                }
                _ => {}
            }
        }
        true
    }
}

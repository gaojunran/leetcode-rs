struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for s in strs {
            let mut sorted = s.clone().into_bytes();
            sorted.sort_unstable();
            map.entry(sorted).or_insert(vec![]).push(s);
        }
        map.into_values().collect()
    }
}

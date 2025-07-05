struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut remap = std::collections::HashMap::new();

        if s.len() != t.len() {
            return false;
        }
        for (chs, cht) in s.chars().zip(t.chars()) {
            // 1. map[chs] -> none  remap[cht] -> none 可以插入
            // 2. map[chs] -> none  remap[cht] != none 直接判断，为假直接false
            // 3. map[chs] != none 直接判断
            if let Some(v) = map.get(&chs) {
                if cht != *v {
                    return false;
                }
            } else if let Some(v) = remap.get(&cht) {
                if chs != *v {
                    return false;
                }
            } else {
                map.insert(chs, cht);
                remap.insert(cht, chs);
            }

            // match (map.get(&cs), remap.get(&ct)) {
            //     (Some(&m), _) if m != ct => return false,
            //     (_, Some(&r)) if r != cs => return false,
            //     (None, None) => {
            //         map.insert(cs, ct);
            //         remap.insert(ct, cs);
            //     }
            //     _ => {}
            // }
        }
        true
    }
}

struct Solution;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            match map.get_mut(n) {
                Some(idx) => {
                    if (i as i32 - *idx as i32).abs() <= k {
                        return true;
                    } else {
                        *idx = i;
                    }
                }
                None => {
                    map.insert(*n, i); // insert 也可以更新，也有返回值，详见简单解法
                }
            }
        }

        // for (idx, num) in nums.into_iter().enumerate() {
        //     let Some(last) = map.insert(num, idx) else {
        //         continue;
        //     };
        //     if idx - last <= k as usize {
        //         return true;
        //     }
        // }
        false
    }
}

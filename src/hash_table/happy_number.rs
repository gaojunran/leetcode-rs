struct Solution;
impl Solution {
    // pub fn is_happy(n: i32) -> bool {
    //     let mut map = std::collections::HashMap::new();
    //     let mut curr = n;
    //     loop {
    //         let sum: i32 = curr
    //             .to_string()
    //             .chars()
    //             .map(|c| c.to_string().parse::<i32>().unwrap().pow(2))
    //             .sum();

    //         let mut bytes = sum.to_string().into_bytes();
    //         bytes.sort_unstable();
    //         if map.contains_key(&bytes) {
    //             return false;
    //         } else {
    //             map.insert(bytes.clone(), true);
    //         }
    //         if sum == 1 {
    //             return true;
    //         }
    //         curr = sum;
    //     }
    // }

    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = std::collections::HashSet::new();

        while n != 1 {
            if !seen.insert(n) {
                return false;
            }
            n = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(2) as i32)
                .sum();
        }

        true
    }
}

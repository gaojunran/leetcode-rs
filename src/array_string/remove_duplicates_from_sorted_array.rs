struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut i = 0;
        let mut j = 0;
        while i < nums.len() {
            if !set.contains(&nums[i]) {
                // 不重复 没见过 不该删除
                set.insert(nums[i]);
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }
        j as i32
    }
}

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut set = std::collections::HashSet::new();
//         let mut j = 0;
//         for i in 0..nums.len() {
//             if set.insert(nums[i]) {
//                 nums[j] = nums[i];
//                 j += 1;
//             }
//         }
//         j as i32
//     }
// }

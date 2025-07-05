struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        // let mut i = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            // if let Some(count) = map.get(nums[i]) {
            //   if count < 2 {
            //     map.insert(nums[i], v)
            //   }
            // }
            map.entry(nums[i])
                .and_modify(|x| {
                    if *x < 2 {
                        nums[j] = nums[i];
                        j += 1;
                    }
                    *x += 1;
                })
                .or_insert_with(|| {
                    nums[j] = nums[i];
                    j += 1;
                    1
                });
        }
        j as i32
    }
}

// 正解：有序数组，快慢指针
// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//       let mut stack_size = 2;
//       for i in 2..nums.len() {
//         if nums[stack_size - 2] != nums[i] {
//             nums[stack_size] = nums[i];
//             stack_size += 1;
//         }
//       }
//       stack_size.min(nums.len()) as i32
//     }
// }

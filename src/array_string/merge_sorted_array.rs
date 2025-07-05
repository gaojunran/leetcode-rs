struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = 0; // nums1 中当前遍历位置
        let mut j = 0; // nums2 中当前遍历位置

        while j < n {
            // 如果 nums1 已遍历完 m+j 个元素，说明 nums2 剩下的直接插入
            if i >= m + j {
                nums1.insert(i as usize, nums2[j as usize]);
                j += 1;
                i += 1;
            } else if nums1[i as usize] <= nums2[j as usize] {
                i += 1;
            } else {
                nums1.insert(i as usize, nums2[j as usize]);
                j += 1;
                i += 1;
            }
        }

        // 最后截断到 m + n 长度
        nums1.truncate((m + n) as usize);
    }
}

// impl Solution {
//     pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//         let (mut i, mut j, mut k) = (m as i32 - 1, n as i32 - 1, (m + n) as i32 - 1);

//         while j >= 0 {
//             if i >= 0 && nums1[i as usize] > nums2[j as usize] {
//                 nums1[k as usize] = nums1[i as usize];
//                 i -= 1;
//             } else {
//                 nums1[k as usize] = nums2[j as usize];
//                 j -= 1;
//             }
//             k -= 1;
//         }
//     }
// }

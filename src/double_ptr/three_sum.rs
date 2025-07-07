struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort(); // 排序
        let mut triples = Vec::new();

        let len = nums.len();
        for i in 0..len {
            // 跳过重复的 nums[i]
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = len - 1;

            while l < r {
                // 跳过重复的 nums[l]
                if l > i + 1 && nums[l] == nums[l - 1] {
                    l += 1;
                    continue;
                }

                // 跳过重复的 nums[r]
                if r < len - 1 && nums[r] == nums[r + 1] {
                    r -= 1;
                    continue;
                }

                // 防止越界和错位
                if l >= r {
                    break;
                }

                let sum = nums[i] + nums[l] + nums[r];
                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    triples.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                }
            }
        }

        triples
    }
}

// struct Solution;

// impl Solution {
//     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut res = Vec::new();
//         let len = nums.len();
//         if len < 3 {
//             return res;
//         }

//         nums.sort_unstable(); // 更快的排序方式

//         for i in 0..len - 2 {
//             // 跳过重复的 nums[i]
//             if i > 0 && nums[i] == nums[i - 1] {
//                 continue;
//             }

//             let (mut l, mut r) = (i + 1, len - 1);

//             while l < r {
//                 let sum = nums[i] + nums[l] + nums[r];
//                 match sum.cmp(&0) {
//                     std::cmp::Ordering::Less => l += 1,
//                     std::cmp::Ordering::Greater => r -= 1,
//                     std::cmp::Ordering::Equal => {
//                         res.push(vec![nums[i], nums[l], nums[r]]);
//                         l += 1;
//                         r -= 1;

//                         // 跳过重复的 nums[l]
//                         while l < r && nums[l] == nums[l - 1] {
//                             l += 1;
//                         }
//                         // 跳过重复的 nums[r]
//                         while l < r && nums[r] == nums[r + 1] {
//                             r -= 1;
//                         }
//                     }
//                 }
//             }
//         }

//         res
//     }
// }

use std::collections::VecDeque;

struct Solution;

// use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num = num_courses as usize;
        let mut in_degree = vec![0; num];
        let mut graph = vec![vec![]; num];

        // 建图 & 统计入度
        for pair in prerequisites {
            let course = pair[0] as usize;
            let prereq = pair[1] as usize;
            graph[prereq].push(course);
            in_degree[course] += 1;
        }

        // 拓扑排序：初始化入度为 0 的节点
        let mut queue = VecDeque::new();
        for i in 0..num {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut order = vec![];

        while let Some(curr) = queue.pop_front() {
            order.push(curr as i32);
            for &next in &graph[curr] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        // 如果排序后的课程数量 < 总课程数，说明存在环，返回空数组
        if order.len() == num { order } else { vec![] }
    }
}

use std::collections::VecDeque;

struct Solution;

// use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

        // 拓扑排序
        let mut queue = VecDeque::new();
        for (i, &deg) in in_degree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(i);
            }
        }

        let mut visited = 0;

        while let Some(curr) = queue.pop_front() {
            visited += 1;
            for &next in &graph[curr] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        visited == num
    }
}

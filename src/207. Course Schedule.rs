
struct Graph {
    edges: Vec<Vec<i32>>,
    visited: Vec<i32>,
    has_cycle: bool,
}

impl Solution {
    fn dfs(u: i32, g: &mut Graph) {
        g.visited[u as usize] = 1;
        let edges = g.edges[u as usize].clone();
        for v in edges.iter() {
            if g.visited[*v as usize] == 0 {
                Self::dfs(*v, g);
                if g.has_cycle {
                    return;
                }
            } else if g.visited[*v as usize] == 1 {
                g.has_cycle = true;
                return;
            }
        }
        g.visited[u as usize] = 2;
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut g: Graph = Graph {
            edges: vec![],
            visited: vec![],
            has_cycle: false,
        };
        g.edges.resize(num_courses as usize, vec![]);
        g.visited.resize(num_courses as usize, 0);
        for prerequisite in prerequisites {
            g.edges[prerequisite[1] as usize].push(prerequisite[0]);
        }
        for i in 0..num_courses {
            if g.has_cycle {
                return false;
            }
            if g.visited[i as usize] == 0 {
                Self::dfs(i, &mut g);
            }
        }
        !g.has_cycle
    }
}
